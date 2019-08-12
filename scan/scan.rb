#!/usr/bin/env ruby

require "rexml/document"
require "fileutils"
require 'pathname'
require_relative 'reflex'
require_relative 'vision'

def render_comment(text, indent = 0)
  lines = text.lines
  lines.shift while lines.first.strip.empty?
  lines.pop while lines.last.strip.empty?
  remove_indent_chars =
    lines
      .select { |line| !line.strip.empty? }
      .map { |line| line.rstrip.gsub(/^(\s*).*$/, "\\1").size }.min
  
  comment = ""
  lines.each do |line|
    comment += "    " * indent + "// " + line.slice(remove_indent_chars..)&.rstrip.to_s + "\n"
  end
  comment
end

def camel_case(text)
  text.split('_').collect(&:capitalize).join
end

class Protocol
  attr_reader :name, :copyright, :interfaces

  def initialize(elem)
    @name = elem.attributes["name"].strip
    elem.select { |child| child.node_type == :element }.each do |child|
      case child.name
      when "copyright"
        raise "Oops! multiple copyright" if @copyright
        @copyright = Copyright.new(child)
      when "interface"
        interface = Interface.new(child, @name)
        next if [
            ["wayland", "wl_shell"],
            ["wayland", "wl_shell_surface"],
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
    @name = elem.attributes["name"].strip
    @protocol_name = protocol_name
    @version = elem.attributes["version"].strip
    @global_singleton = false
    [
      ["wayland", "wl_display", 1],
      ["wayland", "wl_compositor", 2],
      ["wayland", "wl_shm", 3],
      ["wayland", "wl_registry", 4],
      ["wayland", "wl_data_device_manager", 5],
      ["xdg_shell", "xdg_wm_base", 6],
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
      when "description"
        raise "Oops! multiple description" if @description
        @description = Description.new(child)
      when "request"
        @requests ||= []
        @requests << Request.new(child, @requests.size, @name)
        @requests.sort_by!(&:name)
      when "event"
        @events ||= []
        @events << Event.new(child, @events.size)
        @events.sort_by!(&:name)
      when "enum"
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
      "mut "
    else
      ""
    end
  end

  def decode
    result = ""
    error =<<-ERROR
        return context.invalid_method_dispatch(format!(
            "opcode={} args={:?} not found", opcode, args
        ));
    ERROR

    unless @requests
      result += error
      return result
    end

    result +=<<-EOF
        #[allow(unused_mut)] let mut cursor = Cursor::new(&args);
        match opcode {
    EOF

    @requests.sort_by(&:index).each do |request|
      result +=<<-DESERIALIZE
          #{request.index} => {
              #{request.args.map(&:deserialize).join("")}
              if Ok(cursor.position()) != args.len().try_into() {
                  return context.invalid_method_dispatch(format!(
                      "opcode={} args={:?} not found", opcode, args
                  ));
              }
              return Box::new(super::#{camel_case(@name)}::#{request.rust_name}(context#{request.args.map { |arg| ", " + arg.name }.join})
                  .and_then(|(session, next_action)| -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
                      match next_action {
                          NextAction::Nop => Box::new(futures::future::ok(session)),
                          NextAction::Relay => Box::new(futures::future::ok(()).and_then(|_| futures::future::ok(session))),
                          NextAction::RelayWait => Box::new(futures::future::ok(()).and_then(|_| futures::future::ok(())).and_then(|_| futures::future::ok(session))),
                      }
                  })
              );
          },
      DESERIALIZE
    end
    result +=<<-EOF
          _ => {},
        };
    EOF
    result += error
    result
  end

  def decode_vision
    result = ""
    error =<<-ERROR
        return context.invalid_method_dispatch(format!(
            "opcode={} args={:?} not found", opcode, args
        ));
    ERROR

    unless @requests
      result += error
      return result
    end

    result +=<<-EOF
        #[allow(unused_mut)] let mut cursor = Cursor::new(&args);
        match opcode {
    EOF

    @requests.sort_by(&:index).each do |request|
      result +=<<-DESERIALIZE
          #{request.index} => {
              #{request.args.map(&:deserialize_vision).join("")}
              if Ok(cursor.position()) != args.len().try_into() {
                  return context.invalid_method_dispatch(format!(
                      "opcode={} args={:?} not found", opcode, args
                  ));
              }
              return Box::new(super::#{camel_case(@name)}::#{request.rust_name}(context#{request.args.map { |arg| ", " + arg.name }.join})
                  .and_then(|(session, next_action)| -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
                      Box::new(futures::future::ok(session))
                  })
              );
          },
      DESERIALIZE
    end
    result +=<<-EOF
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
    @name = elem.attributes["name"].strip
    @index = index
    @rust_name = @name
    @rust_name += "_fn" if @rust_name == "move"
    @args = []
    encode_offset = "i + 8"
    elem.select { |elem| elem.node_type == :element }.each do |child|
      case child.name
      when "description"
        raise "Oops! multiple description" if @description
        @description = Description.new(child)
      when "arg"
        arg = Arg.create(child, encode_offset, interface_name)
        @args << arg
        encode_offset += " + " + arg.encode_len
      else
        raise "unhandled element: #{child}"
      end
    end
  end
end

class Entry
  attr_reader :name, :summary, :value
  attr_writer :name

  def initialize(elem)
    @name = elem.attributes["name"].strip
    @summary = elem.attributes["summary"]&.strip
    @value = elem.attributes["value"]
  end
end

class Enum
  attr_reader :name, :description, :entries

  def initialize(elem)
    @name = elem.attributes["name"].strip
    @entries = []
    append_enum_prefix = false
    elem.select { |elem| elem.node_type == :element }.each do |child|
      case child.name
      when "description"
        raise "Oops! multiple description" if @description
        @description = Description.new(child)
      when "entry"
        entry = Entry.new(child)
        @entries << entry
        if entry.name =~ /^[0-9]/
          append_enum_prefix = true
        elsif entry.name !~ /^[a-z]/
          raise "invalid entry: " + entry.inspect
        end
      else
        raise "unhandled element: #{child}"
      end
    end
    if append_enum_prefix
      @entries = @entries.map do |entry|
        entry.name = @name + "_" + entry.name
        entry
      end
    end
  end
end

class Arg
  attr_reader :name, :summary, :encode_len, :type, :rust_type, :dynamic_len, :encode_offset, :interface_name

  def self.create(elem, encode_offset, interface_name)
    name = elem.attributes["name"]
    summary = elem.attributes["summary"]
    type = elem.attributes["type"]
    case type
    when "uint"
      UintArg.new(name, summary, type, encode_offset, interface_name)
    when "int"
      IntArg.new(name, summary, type, encode_offset, interface_name)
    when "object"
      ObjectArg.new(name, summary, type, encode_offset, interface_name)
    when "string"
      StringArg.new(name, summary, type, encode_offset, interface_name)
    when "fd"
      FdArg.new(name, summary, type, encode_offset, interface_name)
    when "new_id"
      NewIdArg.new(name, summary, type, encode_offset, interface_name)
    when "fixed"
      FixedArg.new(name, summary, type, encode_offset, interface_name)
    when "array"
      ArrayArg.new(name, summary, type, encode_offset, interface_name)
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
end

class UintArg < Arg
  def initialize(name, summary, type, encode_offset, interface_name)
    @name = name
    @summary = summary
    @encode_len = "4"
    @encode_offset = encode_offset
    @dynamic_len = false
    @type = type
    @rust_type = "u32"
    @interface_name = interface_name
  end

  def serialize
    "NativeEndian::write_u32(&mut dst[#{@encode_offset}..], self.#{name});"
  end

  def deserialize
    <<DESERIAliZE
            let #{name} = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
#{deserialize_return_error}
            };
DESERIAliZE
  end
end

class IntArg < Arg
  def initialize(name, summary, type, encode_offset, interface_name)
    @name = name
    @summary = summary
    @encode_len = "4"
    @encode_offset = encode_offset
    @dynamic_len = false
    @type = type
    @rust_type = "i32"
    @interface_name = interface_name
  end

  def serialize
    "NativeEndian::write_i32(&mut dst[#{@encode_offset}..], self.#{name});"
  end

  def deserialize
    <<DESERIAliZE
            let #{name} = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
#{deserialize_return_error}
            };
DESERIAliZE
  end
end

class ObjectArg < Arg
  def initialize(name, summary, type, encode_offset, interface_name)
    @name = name
    @summary = summary
    @encode_len = "4"
    @encode_offset = encode_offset
    @dynamic_len = false
    @type = type
    @rust_type = "u32"
    @interface_name = interface_name
  end

  def serialize
    "NativeEndian::write_u32(&mut dst[#{@encode_offset}..], self.#{name});"
  end

  def deserialize
    <<DESERIAliZE
            let #{name} = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
#{deserialize_return_error}
            };
DESERIAliZE
  end
end

class StringArg < Arg
  def initialize(name, summary, type, encode_offset, interface_name)
    @name = name
    @summary = summary
    @encode_len = "(4 + (self.#{name}.len() + 1 + 3) / 4 * 4)"
    @encode_offset = encode_offset
    @dynamic_len = true
    @type = type
    @rust_type = "String"
    @interface_name = interface_name
  end

  def serialize
    <<SERIALIZE
        NativeEndian::write_u32(&mut dst[#{@encode_offset}..], (self.#{name}.len() + 1) as u32);
        {
            let mut aligned = self.#{name}.clone();
            aligned.push(0u8.into());
            while aligned.len() % 4 != 0 {
                aligned.push(0u8.into());
            }
            dst[(#{@encode_offset} + 4)..(#{@encode_offset} + 4 + aligned.len())]
                .copy_from_slice(aligned.as_bytes());
        }
SERIALIZE
  end

  def deserialize
    <<DESERIAliZE
            let #{name} = {
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
                let s = if let Ok(x) = String::from_utf8(buf) {
                    x
                } else {
#{deserialize_return_error}
                };
                cursor.set_position(cursor.position() + (padded_buf_len - buf_len) as u64);
                s
            };
DESERIAliZE
  end
end

class FdArg < Arg
  def initialize(name, summary, type, encode_offset, interface_name)
    @name = name
    @summary = summary
    @encode_len = "0"
    @encode_offset = encode_offset
    @dynamic_len = false
    @type = type
    @rust_type = "i32"
    @interface_name = interface_name
  end

  def serialize
    # "NativeEndian::write_i32(&mut dst[#{@encode_offset}..], self.#{name});"
    "// unimplemented!();"
    "println!(\"UNIMPLEMENTED!!!!!\");"
  end

  def deserialize
    <<-DESERIAliZE
      if context.fds.len() == 0 {
          #{deserialize_return_error}
      }
      let #{name} = {
          let rest = context.fds.split_off(1);
          let first = context.fds.pop().expect("fds");
          context.fds = rest;
          first
      };
    DESERIAliZE
  end

  def deserialize_vision
    <<-DESERIAliZE
      let #{name} = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
          x 
      } else {
          #{deserialize_return_error}
      };
    DESERIAliZE
  end
end

class NewIdArg < Arg
  def initialize(name, summary, type, encode_offset, interface_name)
    @name = name
    @summary = summary
    @encode_len = "4"
    @encode_offset = encode_offset
    @dynamic_len = false
    @type = type
    @rust_type = "u32"
    @interface_name = interface_name
  end

  def serialize
    "NativeEndian::write_u32(&mut dst[#{@encode_offset}..], self.#{name});"
  end

  def deserialize
    <<-DESERIAliZE
      let #{name} = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
          x 
      } else {
          #{deserialize_return_error}
      };
    DESERIAliZE
  end
end

class FixedArg < Arg
  def initialize(name, summary, type, encode_offset, interface_name)
    @name = name
    @summary = summary
    @encode_len = "4"
    @encode_offset = encode_offset
    @dynamic_len = false
    @type = type
    @rust_type = "u32"
    @interface_name = interface_name
  end

  def serialize
    "NativeEndian::write_u32(&mut dst[#{@encode_offset}..], self.#{name});"
  end

  def deserialize
    <<DESERIAliZE
            let #{name} = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
#{deserialize_return_error}
            };
DESERIAliZE
  end
end

class ArrayArg < Arg
  def initialize(name, summary, type, encode_offset, interface_name)
    @name = name
    @summary = summary
    @encode_len = "(4 + (self.#{name}.len() + 1 + 3) / 4 * 4)"
    @encode_offset = encode_offset
    @dynamic_len = true
    @type = type
    @rust_type = "Vec<u8>"
    @interface_name = interface_name
  end

  def serialize
    <<SERIALIZE

        NativeEndian::write_u32(&mut dst[#{@encode_offset}..], self.#{name}.len() as u32);
        {
            let mut aligned_#{name} = self.#{name}.clone();
            while aligned_#{name}.len() % 4 != 0 {
                aligned_#{name}.push(0u8);
        }
            dst[(#{@encode_offset} + 4)..(#{@encode_offset} + 4 + aligned_#{name}.len())].copy_from_slice(&aligned_#{name}[..]);
        }
SERIALIZE
  end

  def deserialize
    <<DESERIAliZE
            let #{name} = {
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
    @name = elem.attributes["name"].strip
    @index = index
    @args = []
    encode_offset = "i + 8"
    elem.select { |elem| elem.node_type == :element }.each do |child|
      case child.name
      when "description"
        raise "Oops! multiple description" if @description
        @description = Description.new(child)
      when "arg"
        arg = Arg.create(child, encode_offset, @name)
        @args << arg
        encode_offset += " + " + arg.encode_len
      else
        raise "unhandled element: #{child}"
      end
    end
  end

  def encode
    result =<<FN_ENCODE
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8
FN_ENCODE
    @args.each do |arg|
      result += " + #{arg.encode_len}"
    end
    result += ";\n"
    result += <<HEADER
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let i = dst.len();
        dst.resize(i + total_len, 0);

        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | #{@index}) as u32);

HEADER
    @args.each do |arg|
      result += "        #{arg.serialize}\n"
    end
    result += "        Ok(())\n"
    result += "    }\n"
  end
end

class Description
  attr_reader :text, :summary

  def initialize(elem)
    @summary = elem.attributes["summary"]
    @text = elem.text
  end

  def comment(indent = 0)
    r = "    " * indent + "// #{@summary}\n"
    if @text
      r += "    " * indent + "//\n"
      r += render_comment(@text, indent)
    end
    r
  end
end

protocols = [
  "/usr/share/wayland/wayland.xml",
  "/usr/share/wayland-protocols/stable/xdg-shell/xdg-shell.xml",
].map do |path|
  Protocol.new(
    REXML::Document.new(File.read(path)).elements.find do |elem|
      elem.node_type == :element && elem.name == "protocol"
    end
  )
end.sort_by(&:name)

base_dir = Pathname(__dir__).parent

generate_reflex(base_dir, protocols)
generate_vision(base_dir, protocols)

system("cd '#{base_dir}/reflex' && cargo fmt")
system("cd '#{base_dir}/vision' && cargo fmt")
