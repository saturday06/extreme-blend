def generate_vision(base_dir, protocols)
  open("#{base_dir}/vision/src/protocol.rs", "wb") do |f|
    protocols.each do |protocol|
      FileUtils.mkdir_p("#{base_dir}/vision/src/protocol/#{protocol.name}")
      f.puts("pub mod #{protocol.name};")
    end
    f.puts <<MOD
pub mod codec;
pub mod event;
pub mod request;
pub mod resource;
pub mod session;
MOD
  end

  open("#{base_dir}/vision/src/protocol/resource.rs", "wb") do |f|
    f.puts(<<EOF)
use std::sync::{Arc, RwLock};

pub enum Resource {
EOF
    protocols.each do |protocol|
      protocol.interfaces.each do |interface|
        f.puts("    #{camel_case(interface.name)}(#{interface.receiver_type}),")
      end
    end
    f.puts("}")
    f.puts("")
    f.puts(<<DISPATCH_REQUEST)
pub fn dispatch_request(resource: Resource, session: crate::protocol::session::Session, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
    match resource {
DISPATCH_REQUEST
    protocols.each do |protocol|
      protocol.interfaces.each do |interface|
        f.puts("        Resource::#{camel_case(interface.name)}(object) => {")
        f.puts("            super::#{protocol.name}::#{interface.name}::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)")
        f.puts("        }")
      end
    end
    f.puts(<<DISPATCH_REQUEST)
    }
}
DISPATCH_REQUEST
  end

  protocols.each do |protocol|
    FileUtils.mkdir_p("#{base_dir}/vision/src/protocol/#{protocol.name}")
    open("#{base_dir}/vision/src/protocol/#{protocol.name}.rs", "wb") do |f|
      f.puts(render_comment(protocol.copyright.text))
      f.puts("")
      protocol.interfaces.each do |interface|
        f.puts("pub mod #{interface.name};")
      end
    end

    protocol.interfaces.each do |interface|
      next unless interface.enums
      FileUtils.mkdir_p("#{base_dir}/vision/src/protocol/#{protocol.name}/#{interface.name}")
      open("#{base_dir}/vision/src/protocol/#{protocol.name}/#{interface.name}/enums.rs", "wb") do |f|
        f.puts(render_comment(protocol.copyright.text))
        f.puts("")
        interface.enums.each do |enum|
          f.puts("")
          f.puts(enum.description.comment()) if enum.description
          f.puts("#[allow(dead_code)]")
          f.puts("pub enum #{camel_case(enum.name)} {")
          enum.entries.each do |entry|
            f.puts("    #{camel_case(entry.name)} = #{entry.value}, // #{entry.summary}")
          end
          f.puts("}")
        end
      end
    end

    protocol.interfaces.each do |interface|
      next unless interface.events
      FileUtils.mkdir_p("#{base_dir}/vision/src/protocol/#{protocol.name}/#{interface.name}")
      open("#{base_dir}/vision/src/protocol/#{protocol.name}/#{interface.name}/events.rs", "wb") do |f|
        f.puts(render_comment(protocol.copyright.text))
        f.puts("")
        f.puts("use byteorder::{ByteOrder, NativeEndian};")

        interface.events.each do |event|
          f.puts("")
          f.puts(event.description.comment())
          f.puts("#[allow(dead_code)]")
          f.puts("pub struct #{camel_case(event.name)} {")
          f.puts("    pub sender_object_id: u32,")
          event.args.each do |arg|
            f.puts("    pub #{arg.name}: #{arg.rust_type}, // #{arg.type}: #{arg.summary}")
          end
          f.puts("}")
          f.puts("")
          f.puts("impl super::super::super::event::Event for #{camel_case(event.name)} {")
          f.puts(event.encode)
          f.puts("}")
        end
      end
    end

    protocol.interfaces.each do |interface|
      FileUtils.mkdir_p("#{base_dir}/vision/src/protocol/#{protocol.name}/#{interface.name}")
      open("#{base_dir}/vision/src/protocol/#{protocol.name}/#{interface.name}/lib.rs", "wb") do |f|
        f.puts(render_comment(protocol.copyright.text))
        f.puts(<<USE)

#[allow(unused_imports)] use byteorder::{ByteOrder, NativeEndian, ReadBytesExt};
#[allow(unused_imports)] use futures::future::Future;
#[allow(unused_imports)] use futures::sink::Sink;
#[allow(unused_imports)] use std::convert::TryInto;
#[allow(unused_imports)] use std::io::{Cursor, Read};
#[allow(unused_imports)] use std::sync::{Arc, RwLock};

USE

        if interface.global_singleton
          f.puts("#[allow(dead_code)]")
          f.puts("pub const GLOBAL_SINGLETON_NAME: u32 = #{interface.global_singleton_name_int};")
        end
        f.puts(<<CODE)
#[allow(dead_code)]
pub const VERSION: u32 = #{interface.version};

#[allow(unused_variables)]
#[allow(dead_code)]
CODE
        f.puts("pub fn dispatch_request(context: crate::protocol::session::Context<#{interface.receiver_type}>, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {")
        f.puts(interface.decode_vision)
        f.puts("}")
        f.puts(<<INTO)

impl Into<crate::protocol::resource::Resource> for #{interface.receiver_type} {
    fn into(self) -> crate::protocol::resource::Resource {
        crate::protocol::resource::Resource::#{camel_case(interface.name)}(self)
    }
}
INTO
      end
    end

    next if ARGV[0] != "--overwrite-protocols"
    
    protocol.interfaces.each do |interface|
      open("#{base_dir}/vision/src/protocol/#{protocol.name}/#{interface.name}.rs", "wb") do |f|
        f.puts(render_comment(protocol.copyright.text))
        f.puts("")
        f.puts(<<USE)
#[allow(unused_imports)] use crate::protocol::session::{Context, Session};
#[allow(unused_imports)] use futures::future::{err, ok, Future};
#[allow(unused_imports)] use futures::sink::Sink;
#[allow(unused_imports)] use std::sync::{Arc, RwLock};

USE
        if interface.enums
          f.puts("pub mod enums;")
        end
        if interface.events
          f.puts("pub mod events;")
        end
        f.puts("mod lib;")
        f.puts("pub use lib::*;")
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
            f.puts("        context: Context<#{interface.short_receiver_type}>,")
            request.args.each do |arg|
              f.print("        _#{arg.name}: #{arg.rust_type}, // #{arg.type}: #{arg.summary}\n")
            end
            f.puts(<<FUNC_BODY)
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        context.invalid_method("#{interface.name}::#{request.name} is not implemented yet".to_string())
    }
FUNC_BODY
          end
          f.puts("}")
        end
      end
    end
  end
end
