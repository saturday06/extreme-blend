#!/usr/bin/env ruby
# frozen_string_literal: true

require 'rexml/document'
require 'fileutils'
require 'pathname'
require_relative 'reflex'
require_relative 'vision'

def render_comment(text, indent = 0)
  lines = text.lines
  lines.shift while lines.first.strip.empty?
  lines.pop while lines.last.strip.empty?
  remove_indent_chars =
    lines
    .reject { |line| line.strip.empty? }
    .map { |line| line.rstrip.gsub(/^(\s*).*$/, '\\1').size }.min

  comment = ''
  lines.each do |line|
    comment += '    ' * indent + '// ' + line.slice(remove_indent_chars..)&.rstrip.to_s + "\n"
  end
  comment
end

def camel_case(text)
  text.split('_').collect(&:capitalize).join
end

class Protocol
  attr_reader :name, :copyright, :interfaces

  def initialize(elem)
    @name = elem.attributes['name'].strip
    elem.select { |child| child.node_type == :element }.each do |child|
      case child.name
      when 'copyright'
        raise 'Oops! multiple copyright' if @copyright

        @copyright = Copyright.new(child)
      when 'interface'
        interface = Interface.new(child, @name)
        next if [
          %w[wayland wl_shell],
          %w[wayland wl_shell_surface]
        ].find { |protocol_name, interface_name| protocol_name == @name && interface_name == interface.name }

        @interfaces ||= []
        @interfaces << interface
        @interfaces.sort_by!(&:name)
      else
        raise "unhandled element: #{child}"
      end
    end
  end
end

class Copyright
  attr_reader :text

  def initialize(elem)
    @text = elem.text
  end
end

class Interface
  attr_reader :name, :description, :requests, :events, :enums, :version, :receiver_type, :protocol_name, :global_singleton, :short_receiver_type, :global_singleton_name_int

  def initialize(elem, protocol_name)
    @name = elem.attributes['name'].strip
    @protocol_name = protocol_name
    @version = elem.attributes['version'].strip
    @global_singleton = false
    [
      ['wayland', 'wl_display', 1],
      ['wayland', 'wl_compositor', 2],
      ['wayland', 'wl_shm', 3],
      ['wayland', 'wl_registry', 4],
      ['wayland', 'wl_data_device_manager', 5],
      ['xdg_shell', 'xdg_wm_base', 6]
    ].each do |protocol_name, name, name_int|
      if protocol_name == @protocol_name && name == @name
        @global_singleton = true
        @global_singleton_name_int = name_int
      end
    end

    if @global_singleton
      @short_receiver_type = "Arc<RwLock<#{camel_case(@name)}>>"
      @receiver_type = "Arc<RwLock<crate::protocol::#{@protocol_name}::#{@name}::#{camel_case(@name)}>>"
    else
      @short_receiver_type = camel_case(@name)
      @receiver_type = "crate::protocol::#{@protocol_name}::#{@name}::#{camel_case(@name)}"
    end
    elem.select { |elem| elem.node_type == :element }.each do |child|
      case child.name
      when 'description'
        raise 'Oops! multiple description' if @description

        @description = Description.new(child)
      when 'request'
        @requests ||= []
        @requests << Request.new(child, @requests.size, @name)
        @requests.sort_by!(&:name)
      when 'event'
        @events ||= []
        @events << Event.new(child, @events.size)
        @events.sort_by!(&:name)
      when 'enum'
        @enums ||= []
        @enums << Enum.new(child)
        @enums.sort_by!(&:name)
      else
        raise "unhandled element: #{child}"
      end
    end
  end

  def dispatch_context_mut
    fd_arg = (@requests || []).find do |request|
      request.args.find do |arg|
        arg.is_a?(FdArg)
      end
    end
    if fd_arg
      'mut '
    else
      ''
    end
  end

  def decode
    result = ''
    error = <<-ERROR
        return context.invalid_method_dispatch(format!(
            "opcode={} args={:?} not found", opcode, args
        ));
    ERROR

    unless @requests
      result += error
      return result
    end

    result += <<-EOF
        let sender_object_id = context.sender_object_id;
        #[allow(unused_mut)] let mut cursor = Cursor::new(&args);
        match opcode {
    EOF

    @requests.sort_by(&:index).each do |request|
      result += <<-DESERIALIZE
          #{request.index} => {
              #{request.args.map(&:deserialize).join('')}
              if Ok(cursor.position()) != args.len().try_into() {
                  return context.invalid_method_dispatch(format!(
                      "opcode={} args={:?} not found", opcode, args
                  ));
              }
              let relay_buf = #{request.encode_vision};
              return Box::new(super::#{camel_case(@name)}::#{request.rust_name}(context#{request.args.map { |arg| ', arg_' + arg.name }.join})
                  .and_then(|(session, next_action)| -> Box<dyn futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
                      match next_action {
                          NextAction::Nop => Box::new(futures::future::ok(session)),
                          NextAction::Relay => session.relay(relay_buf),
                          NextAction::RelayWait => session.relay_wait(relay_buf),
                      }
                  })
              );
          },
      DESERIALIZE
    end
    result += <<-EOF
          _ => {},
        };
    EOF
    result += error
    result
  end

  def decode_vision
    result = ''
    error = <<-ERROR
        return context.invalid_method_dispatch(format!(
            "opcode={} args={:?} not found", opcode, args
        ));
    ERROR

    unless @requests
      result += error
      return result
    end

    result += <<-EOF
        #[allow(unused_mut)] let mut cursor = Cursor::new(&args);
        match opcode {
    EOF

    @requests.sort_by(&:index).each do |request|
      result += <<-DESERIALIZE
          #{request.index} => {
              #{request.args.map(&:deserialize_vision).join('')}
              if Ok(cursor.position()) != args.len().try_into() {
                  return context.invalid_method_dispatch(format!(
                      "opcode={} args={:?} not found", opcode, args
                  ));
              }
              return Box::new(super::#{camel_case(@name)}::#{request.rust_name}(context#{request.args.map { |arg| ', arg_' + arg.name }.join})
                  .and_then(|(session, next_action)| -> Box<dyn futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
                      Box::new(futures::future::ok(session))
                  })
              );
          },
      DESERIALIZE
    end
    result += <<-EOF
          _ => {},
        };
    EOF
    result += error
    result
  end
end

class Request
  attr_reader :name, :description, :args, :rust_name, :index

  def initialize(elem, index, interface_name)
    @name = elem.attributes['name'].strip
    @index = index
    @rust_name = @name
    @rust_name += '_fn' if @rust_name == 'move'
    @args = []
    elem.select { |elem| elem.node_type == :element }.each do |child|
      case child.name
      when 'description'
        raise 'Oops! multiple description' if @description

        @description = Description.new(child)
      when 'arg'
        arg = Arg.create(child, interface_name)
        @args << arg
      else
        raise "unhandled element: #{child}"
      end
    end
  end

  def encode_vision
    result = <<FN_ENCODE
    {
        let total_len = 8
FN_ENCODE
    @args.each do |arg|
      result += " + #{arg.serialize_vision_len}"
    end
    result += ";\n"
    result += <<HEADER
        if total_len > 0xffff {
            println!("Oops! total_len={}", total_len);
            return Box::new(futures::future::err(()));
        }

        let mut dst: Vec<u8> = Vec::new();
        dst.resize(total_len, 0);

        NativeEndian::write_u32(&mut dst[0..], sender_object_id);
        NativeEndian::write_u32(&mut dst[4..], (total_len << 16) as u32 | u32::from(opcode));

        #[allow(unused_mut)] let mut encode_offset = 8;

HEADER
    @args.each do |arg|
      result += <<-ARG
          #{arg.serialize_vision}
          encode_offset += #{arg.serialize_vision_len};
      ARG
    end
    result += <<-FOOTER
            let _ = encode_offset;
            dst
        }
    FOOTER
    result
  end
end

class Entry
  attr_reader :name, :summary, :value
  attr_writer :name

  def initialize(elem)
    @name = elem.attributes['name'].strip
    @summary = elem.attributes['summary']&.strip
    @value = elem.attributes['value']
  end
end

class Enum
  attr_reader :name, :description, :entries

  def initialize(elem)
    @name = elem.attributes['name'].strip
    @entries = []
    append_enum_prefix = false
    elem.select { |elem| elem.node_type == :element }.each do |child|
      case child.name
      when 'description'
        raise 'Oops! multiple description' if @description

        @description = Description.new(child)
      when 'entry'
        entry = Entry.new(child)
        @entries << entry
        if entry.name =~ /^[0-9]/
          append_enum_prefix = true
        elsif entry.name !~ /^[a-z]/
          raise 'invalid entry: ' + entry.inspect
        end
      else
        raise "unhandled element: #{child}"
      end
    end
    if append_enum_prefix
      @entries = @entries.map do |entry|
        entry.name = @name + '_' + entry.name
        entry
      end
    end
  end
end

class Arg
  attr_reader :name, :summary, :serialize_len, :type, :rust_type, :dynamic_len, :interface_name

  def self.create(elem, interface_name)
    name = elem.attributes['name']
    summary = elem.attributes['summary']
    type = elem.attributes['type']
    case type
    when 'uint'
      UintArg.new(name, summary, type, interface_name)
    when 'int'
      IntArg.new(name, summary, type, interface_name)
    when 'object'
      ObjectArg.new(name, summary, type, interface_name)
    when 'string'
      StringArg.new(name, summary, type, interface_name)
    when 'fd'
      FdArg.new(name, summary, type, interface_name)
    when 'new_id'
      NewIdArg.new(name, summary, type, interface_name)
    when 'fixed'
      FixedArg.new(name, summary, type, interface_name)
    when 'array'
      ArrayArg.new(name, summary, type, interface_name)
    else
      raise "unhandled type: #{@type}"
    end
  end

  def deserialize_return_error
    <<-EOF
      return context.invalid_method_dispatch(format!(
        "opcode={} args={:?} not found", opcode, args
      ))
    EOF
  end

  def deserialize_vision
    deserialize
  end

  def serialize_vision_len
    serialize_len
  end

  def serialize_vision
    serialize('arg_')
  end
end

class UintArg < Arg
  def initialize(name, summary, type, interface_name)
    @name = name
    @summary = summary
    @serialize_len = '4'
    @dynamic_len = false
    @type = type
    @rust_type = 'u32'
    @interface_name = interface_name
  end

  def serialize(prefix = 'self.')
    "NativeEndian::write_u32(&mut dst[encode_offset..], #{prefix}#{name});"
  end

  def deserialize
    <<~DESERIAliZE
                  let arg_#{name} = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                      x
                  } else {
      #{deserialize_return_error}
                  };
    DESERIAliZE
  end
end

class IntArg < Arg
  def initialize(name, summary, type, interface_name)
    @name = name
    @summary = summary
    @serialize_len = '4'
    @dynamic_len = false
    @type = type
    @rust_type = 'i32'
    @interface_name = interface_name
  end

  def serialize(prefix = 'self.')
    "NativeEndian::write_i32(&mut dst[encode_offset..], #{prefix}#{name});"
  end

  def deserialize
    <<~DESERIAliZE
                  let arg_#{name} = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                      x
                  } else {
      #{deserialize_return_error}
                  };
    DESERIAliZE
  end
end

class ObjectArg < Arg
  def initialize(name, summary, type, interface_name)
    @name = name
    @summary = summary
    @serialize_len = '4'
    @dynamic_len = false
    @type = type
    @rust_type = 'u32'
    @interface_name = interface_name
  end

  def serialize(prefix = 'self.')
    "NativeEndian::write_u32(&mut dst[encode_offset..], #{prefix}#{name});"
  end

  def deserialize
    <<~DESERIAliZE
                  let arg_#{name} = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                      x
                  } else {
      #{deserialize_return_error}
                  };
    DESERIAliZE
  end
end

class StringArg < Arg
  attr_reader :serialize_vision_len

  def initialize(name, summary, type, interface_name)
    @name = name
    @summary = summary
    @serialize_len =        "{4 + (self.#{name}.len() + 1 + 3) / 4 * 4}"
    @serialize_vision_len = "{4 + ( arg_#{name}.len() + 1 + 3) / 4 * 4}"
    @dynamic_len = true
    @type = type
    @rust_type = 'String'
    @interface_name = interface_name
  end

  def serialize(prefix = 'self.')
    <<SERIALIZE
        NativeEndian::write_u32(&mut dst[encode_offset..], (#{prefix}#{name}.len() + 1) as u32);
        {
            let mut aligned = #{prefix}#{name}.clone();
            aligned.push(0u8.into());
            while aligned.len() % 4 != 0 {
                aligned.push(0u8.into());
            }
            dst[(encode_offset + 4)..(encode_offset + 4 + aligned.len())]
                .copy_from_slice(aligned.as_bytes());
        }
SERIALIZE
  end

  def deserialize
    <<~DESERIAliZE
                  let arg_#{name} = {
                      let buf_len = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                          x
                      } else {
      #{deserialize_return_error}
                      };
                      let padded_buf_len = (buf_len + 3) / 4 * 4;
                      let mut buf = Vec::new();
                      buf.resize(buf_len as usize, 0);
                      if cursor.read_exact(&mut buf).is_err() {
      #{deserialize_return_error}
                      }
                      let s = if let Ok(x) = String::from_utf8(buf) {
                          x
                      } else {
      #{deserialize_return_error}
                      };
                      cursor.set_position(cursor.position() + u64::from(padded_buf_len - buf_len));
                      s
                  };
    DESERIAliZE
  end
end

class FdArg < Arg
  def initialize(name, summary, type, interface_name)
    @name = name
    @summary = summary
    @serialize_len = '0'
    @dynamic_len = false
    @type = type
    @rust_type = 'i32'
    @interface_name = interface_name
  end

  def serialize_vision_len
    '4'
  end

  def serialize_vision
    "NativeEndian::write_i32(&mut dst[encode_offset..], arg_#{name});"
  end

  def serialize(_prefix = 'self.')
    # "NativeEndian::write_i32(&mut dst[encode_offset..], self.#{name});"
    '// unimplemented!();'
    'println!("UNIMPLEMENTED!!!!!");'
  end

  def deserialize
    <<-DESERIAliZE
      if context.fds.is_empty() {
          #{deserialize_return_error}
      }
      let arg_#{name} = {
          let rest = context.fds.split_off(1);
          let first = context.fds.pop().expect("fds");
          context.fds = rest;
          first
      };
    DESERIAliZE
  end

  def deserialize_vision
    <<-DESERIAliZE
      let arg_#{name} = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
          x
      } else {
          #{deserialize_return_error}
      };
    DESERIAliZE
  end
end

class NewIdArg < Arg
  def initialize(name, summary, type, interface_name)
    @name = name
    @summary = summary
    @serialize_len = '4'
    @dynamic_len = false
    @type = type
    @rust_type = 'u32'
    @interface_name = interface_name
  end

  def serialize(prefix = 'self.')
    "NativeEndian::write_u32(&mut dst[encode_offset..], #{prefix}#{name});"
  end

  def deserialize
    <<-DESERIAliZE
      let arg_#{name} = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
          x
      } else {
          #{deserialize_return_error}
      };
    DESERIAliZE
  end
end

class FixedArg < Arg
  def initialize(name, summary, type, interface_name)
    @name = name
    @summary = summary
    @serialize_len = '4'
    @dynamic_len = false
    @type = type
    @rust_type = 'u32'
    @interface_name = interface_name
  end

  def serialize(prefix = 'self.')
    "NativeEndian::write_u32(&mut dst[encode_offset..], #{prefix}#{name});"
  end

  def deserialize
    <<~DESERIAliZE
                  let arg_#{name} = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                      x
                  } else {
      #{deserialize_return_error}
                  };
    DESERIAliZE
  end
end

class ArrayArg < Arg
  attr_reader :serialize_vision_len

  def initialize(name, summary, type, interface_name)
    @name = name
    @summary = summary
    @serialize_len =        "{4 + (self.#{name}.len() + 1 + 3) / 4 * 4}"
    @serialize_vision_len = "{4 + ( arg_#{name}.len() + 1 + 3) / 4 * 4}"
    @dynamic_len = true
    @type = type
    @rust_type = 'Vec<u8>'
    @interface_name = interface_name
  end

  def serialize(prefix = 'self.')
    <<SERIALIZE

        NativeEndian::write_u32(&mut dst[encode_offset..], #{prefix}#{name}.len() as u32);
        {
            let mut aligned_#{name} = #{prefix}#{name}.clone();
            while aligned_#{name}.len() % 4 != 0 {
                aligned_#{name}.push(0u8);
            }
            dst[(encode_offset + 4)..(encode_offset + 4 + aligned_#{name}.len())].copy_from_slice(&aligned_#{name}[..]);
        }
SERIALIZE
  end

  def deserialize
    <<~DESERIAliZE
                  let arg_#{name} = {
                      let buf_len = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                          x
                      } else {
      #{deserialize_return_error}
                      };
                      let padded_buf_len = (buf_len + 3) / 4 * 4;
                      let mut buf = Vec::new();
                      buf.resize(buf_len as usize, 0);
                      if let Err(_) = cursor.read_exact(&mut buf) {
      #{deserialize_return_error}
                      }
                      cursor.set_position(cursor.position() + (padded_buf_len - buf_len) as u64);
                      buf
                  };
    DESERIAliZE
  end
end

class Event
  attr_reader :name, :description, :args, :index

  def initialize(elem, index)
    @name = elem.attributes['name'].strip
    @index = index
    @args = []
    elem.select { |elem| elem.node_type == :element }.each do |child|
      case child.name
      when 'description'
        raise 'Oops! multiple description' if @description

        @description = Description.new(child)
      when 'arg'
        arg = Arg.create(child, @name)
        @args << arg
      else
        raise "unhandled element: #{child}"
      end
    end
  end

  def encode
    result = <<FN_ENCODE
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8
FN_ENCODE
    @args.each do |arg|
      result += " + #{arg.serialize_len}"
    end
    result += ";\n"
    result += <<HEADER
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let mut encode_offset = dst.len();
        dst.resize(encode_offset + total_len, 0);

        NativeEndian::write_u32(&mut dst[encode_offset..], self.sender_object_id);
        let event_opcode = #{@index};
        NativeEndian::write_u32(&mut dst[encode_offset + 4..], ((total_len << 16) | event_opcode) as u32);

        encode_offset += 8;
HEADER
    @args.each do |arg|
      result += <<-ARG
          #{arg.serialize}
          encode_offset += #{arg.serialize_len};
      ARG
    end
    result += <<-FOOTER
          let _ = encode_offset;
          Ok(())
      }
    FOOTER
    result
  end
end

class Description
  attr_reader :text, :summary

  def initialize(elem)
    @summary = elem.attributes['summary']
    @text = elem.text
  end

  def comment(indent = 0)
    r = '    ' * indent + "// #{@summary}\n"
    if @text
      r += '    ' * indent + "//\n"
      r += render_comment(@text, indent)
    end
    r
  end
end

protocols = [
  '/usr/share/wayland/wayland.xml',
  '/usr/share/wayland-protocols/stable/xdg-shell/xdg-shell.xml'
].map do |path|
  Protocol.new(
    REXML::Document.new(File.read(path)).elements.find do |elem|
      elem.node_type == :element && elem.name == 'protocol'
    end
  )
end.sort_by(&:name)

base_dir = Pathname(__dir__).parent

generate_reflex(base_dir, protocols)
generate_vision(base_dir, protocols)

system("cd '#{base_dir}/reflex' && cargo fmt")
system("cd '#{base_dir}/vision' && cargo fmt")
