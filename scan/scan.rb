#!/usr/bin/env ruby

require "rexml/document"
require "fileutils"

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
        @interfaces ||= []
        @interfaces << Interface.new(child)
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
  attr_reader :name, :description, :requests, :events, :enums

  def initialize(elem)
    @name = elem.attributes["name"].strip
    elem.select { |elem| elem.node_type == :element }.each do |child|
      case child.name
      when "description"
        raise "Oops! multiple description" if @description
        @description = Description.new(child)
      when "request"
        @requests ||= []
        @requests << Request.new(child, @requests.size)
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

  
  def decode
    result = ""
    if @requests
      result +=<<EOF
    let mut cursor = Cursor::new(&args);
    match opcode {
EOF
      @requests.sort_by(&:index).each do |request|
        result += "        #{request.index} => {\n"
        result += request.args.map(&:deserialize).join("")
        result += "            return #{camel_case(@name)}::#{request.rust_name}(request, session, tx, sender_object_id, "
        result += request.args.map(&:name).join(", ")
        result += ")\n"
        result += "        },\n"
      end
      result +=<<EOF
        _ => {},
    };
EOF
    end
    result += "    Box::new(futures::future::ok(session))"
    result
  end
end

class Request
  attr_reader :name, :description, :args, :rust_name, :index

  def initialize(elem, index)
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
        arg = Arg.create(child, encode_offset)
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
  attr_reader :name, :summary, :encode_len, :type, :rust_type, :dynamic_len, :encode_offset

  def self.create(elem, encode_offset)
    name = elem.attributes["name"]
    summary = elem.attributes["summary"]
    type = elem.attributes["type"]
    case type
    when "uint"
      UintArg.new(name, summary, type, encode_offset)
    when "int"
      IntArg.new(name, summary, type, encode_offset)
    when "object"
      ObjectArg.new(name, summary, type, encode_offset)
    when "string"
      StringArg.new(name, summary, type, encode_offset)
    when "fd"
      FdArg.new(name, summary, type, encode_offset)
    when "new_id"
      NewIdArg.new(name, summary, type, encode_offset)
    when "fixed"
      FixedArg.new(name, summary, type, encode_offset)
    when "array"
      ArrayArg.new(name, summary, type, encode_offset)
    else
      raise "unhandled type: #{@type}"
    end
  end

  def deserialize_return_error
    <<EOF
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));
EOF
  end
end

class UintArg < Arg
  def initialize(name, summary, type, encode_offset)
    @name = name
    @summary = summary
    @encode_len = "4"
    @encode_offset = encode_offset
    @dynamic_len = false
    @type = type
    @rust_type = "u32"
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
  def initialize(name, summary, type, encode_offset)
    @name = name
    @summary = summary
    @encode_len = "4"
    @encode_offset = encode_offset
    @dynamic_len = false
    @type = type
    @rust_type = "i32"
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
  def initialize(name, summary, type, encode_offset)
    @name = name
    @summary = summary
    @encode_len = "4"
    @encode_offset = encode_offset
    @dynamic_len = false
    @type = type
    @rust_type = "u32"
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
  def initialize(name, summary, type, encode_offset)
    @name = name
    @summary = summary
    @encode_len = "(4 + (self.#{name}.len() + 1 + 3) / 4 * 4)"
    @encode_offset = encode_offset
    @dynamic_len = true
    @type = type
    @rust_type = "String"
  end

  def serialize
    <<SERIALIZE

            NativeEndian::write_u32(&mut dst[#{@encode_offset}..], self.#{name}.len() as u32);
            let mut aligned_#{name} = self.#{name}.clone();
            aligned_#{name}.push(0u8.into());
            while aligned_#{name}.len() % 4 != 0 {
                aligned_#{name}.push(0u8.into());
            }
            dst[(#{@encode_offset} + 4)..(#{@encode_offset} + 4 + aligned_#{name}.len())].copy_from_slice(aligned_#{name}.as_bytes());
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
  def initialize(name, summary, type, encode_offset)
    @name = name
    @summary = summary
    @encode_len = "4"
    @encode_offset = encode_offset
    @dynamic_len = false
    @type = type
    @rust_type = "i32"
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

class NewIdArg < Arg
  def initialize(name, summary, type, encode_offset)
    @name = name
    @summary = summary
    @encode_len = "4"
    @encode_offset = encode_offset
    @dynamic_len = false
    @type = type
    @rust_type = "u32"
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

class FixedArg < Arg
  def initialize(name, summary, type, encode_offset)
    @name = name
    @summary = summary
    @encode_len = "4"
    @encode_offset = encode_offset
    @dynamic_len = false
    @type = type
    @rust_type = "u32"
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
  def initialize(name, summary, type, encode_offset)
    @name = name
    @summary = summary
    @encode_len = "(4 + (self.#{name}.len() + 1 + 3) / 4 * 4)"
    @encode_offset = encode_offset
    @dynamic_len = true
    @type = type
    @rust_type = "Vec<u8>"
  end

  def serialize
    <<SERIALIZE

            NativeEndian::write_u32(&mut dst[#{@encode_offset}..], self.#{name}.len() as u32);
            let mut aligned_#{name} = self.#{name}.clone();
            while aligned_#{name}.len() % 4 != 0 {
                aligned_#{name}.push(0u8);
            }
            dst[(#{@encode_offset} + 4)..(#{@encode_offset} + 4 + aligned_#{name}.len())].copy_from_slice(&aligned_#{name}[..]);
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
        arg = Arg.create(child, encode_offset)
        @args << arg
        encode_offset += " + " + arg.encode_len
      else
        raise "unhandled element: #{child}"
      end
    end
  end

  def encode
    result = "        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {\n"    
    result += "            let total_len = 8"
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
      result += "            #{arg.serialize}\n"
    end
    result += "            Ok(())\n"
    result += "        }\n"
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

open("../reflex/src/protocol.rs", "wb") do |f|
  protocols.each do |protocol|
    FileUtils.mkdir_p("../reflex/src/protocol/#{protocol.name}")
    f.puts("pub mod #{protocol.name};")
  end
  f.puts("")
  f.puts("pub mod codec;")
  f.puts("pub mod event;")
  f.puts("pub mod request;")
  f.puts("pub mod resource;")
  f.puts("pub mod session;")
end

open("../reflex/src/protocol/resource.rs", "wb") do |f|
  f.puts(<<EOF)

use std::sync::{Arc, RwLock};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub enum Resource {
EOF
  protocols.each do |protocol|
    protocol.interfaces.each do |interface|
      f.puts("    #{camel_case(interface.name)}(Arc<RwLock<super::#{protocol.name}::#{interface.name}::#{camel_case(interface.name)}>>),")
    end
  end
  f.puts("}")
  f.puts("")
  f.puts(<<DISPATCH_REQUEST)
pub fn dispatch_request(resource: Resource, session: crate::protocol::session::Session, tx: tokio::sync::mpsc::Sender<Box<super::event::Event + Send>>, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
    match resource {
DISPATCH_REQUEST
  protocols.each do |protocol|
    protocol.interfaces.each do |interface|
      f.puts("        Resource::#{camel_case(interface.name)}(object) => {")
      f.puts("            super::#{protocol.name}::#{interface.name}::dispatch_request(object, session, tx, sender_object_id, opcode, args)")
      f.puts("        }")
    end
  end
  f.puts(<<DISPATCH_REQUEST)
    }
}
DISPATCH_REQUEST
end

protocols.each do |protocol|
  FileUtils.mkdir_p("../reflex/src/protocol/#{protocol.name}")
  open("../reflex/src/protocol/#{protocol.name}.rs", "wb") do |f|
    f.puts(render_comment(protocol.copyright.text))
    f.puts("")
    protocol.interfaces.each do |interface|
      f.puts("pub mod #{interface.name};")
    end
  end

  protocol.interfaces.each do |interface|
    open("../reflex/src/protocol/#{protocol.name}/#{interface.name}.rs", "wb") do |f|
      f.puts(render_comment(protocol.copyright.text))
      f.puts(<<EOF)

#[allow(unused_imports)] use byteorder::{NativeEndian, ReadBytesExt};
#[allow(unused_imports)] use futures::future::Future;
#[allow(unused_imports)] use futures::sink::Sink;
#[allow(unused_imports)] use std::io::{Cursor, Read};
#[allow(unused_imports)] use std::sync::{Arc, RwLock};
EOF
      if interface.enums
        f.puts("")
        f.print("pub mod enums {")
        interface.enums.each do |enum|
          f.puts("")
          f.puts(enum.description.comment(1)) if enum.description
          f.puts("    pub enum #{camel_case(enum.name)} {")
          enum.entries.each do |entry|
            f.puts("        #{camel_case(entry.name)} = #{entry.value}, // #{entry.summary}")
          end
          f.puts("    }")
        end
        f.puts("}")
      end
      if interface.events
        f.puts("")
        f.puts("pub mod events {")
        f.puts("    use byteorder::{ByteOrder, NativeEndian};")
        interface.events.each do |event|
          f.puts("")
          f.puts(event.description.comment(1))
          f.puts("    pub struct #{camel_case(event.name)} {")
          f.puts("        pub sender_object_id: u32,")
          event.args.each do |arg|
            f.puts("        pub #{arg.name}: #{arg.rust_type}, // #{arg.type}: #{arg.summary}")
          end
          f.puts("    }")
          f.puts("")
          f.puts("    impl super::super::super::event::Event for #{camel_case(event.name)} {")
          f.puts(event.encode)
          f.puts("    }")
        end
        f.puts("}")
      end
      f.puts("")
      f.puts("pub fn dispatch_request(request: Arc<RwLock<#{camel_case(interface.name)}>>, session: crate::protocol::session::Session, tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {")
      f.puts(interface.decode)
      f.puts("}")
      f.puts("")
      f.puts(interface.description.comment())
      f.puts("pub struct #{camel_case(interface.name)} {")
      f.puts("}")
      f.puts("")
      if interface.requests
        f.puts("impl #{camel_case(interface.name)} {")
        interface.requests.each_with_index do |request, index|
          f.puts("") if index > 0
          f.puts(request.description.comment(1))
          f.puts("    pub fn #{request.rust_name}(")
          f.puts("        request: Arc<RwLock<#{camel_case(interface.name)}>>,")
          f.puts("        session: crate::protocol::session::Session,")
          f.puts("        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,")
          f.puts("        sender_object_id: u32,")
          request.args.each do |arg|
            f.print("        #{arg.name}: #{arg.rust_type}, // #{arg.type}: #{arg.summary}\n")
          end
          f.puts("    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {")
          f.puts("        Box::new(futures::future::ok(session))")
          f.puts("    }")
        end
        f.puts("}")
      end
    end
  end
end
