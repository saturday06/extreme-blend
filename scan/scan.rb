#!/usr/bin/env ruby

require "rexml/document"
require "fileutils"

def render_comment(text)
  lines = text.lines
  lines.shift while lines.first.strip.empty?
  lines.pop while lines.last.strip.empty?
  remove_indent_chars =
    lines
      .select { |line| !line.strip.empty? }
      .map { |line| line.rstrip.gsub(/^(\s*).*$/, "\\1").size }.min
  
  comment = ""
  lines.each do |line|
    comment += "// " + line.slice(remove_indent_chars..)&.rstrip.to_s + "\n"
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

class Description
  attr_reader :text

  def initialize(elem)
    @text = elem.text
  end
end

class Interface
  attr_reader :name, :description, :request, :events, :enums

  def initialize(elem)
    @name = elem.attributes["name"].strip
    elem.select { |elem| elem.node_type == :element }.each do |child|
      case child.name
      when "description"
        raise "Oops! multiple description" if @description
        @description = Description.new(child)
      when "request"
        @requests ||= []
        @requests << Request.new(child)
        @requests.sort_by!(&:name)
      when "event"
        @events ||= []
        @events << Event.new(child)
        @events.sort_by!(&:name)
      when "enum"
        @enums ||= []
        @enums << Enum.new(child)
        @enums.sort_by!(&:name)
      end
    end  
  end
end

class Request
  attr_reader :name

  def initialize(elem)
    @name = elem.attributes["name"].strip
  end
end

class Enum
  attr_reader :name

  def initialize(elem)
    @name = elem.attributes["name"].strip
  end
end

class Event
  attr_reader :name

  def initialize(elem)
    @name = elem.attributes["name"].strip
  end
end

class Description
  attr_reader :text

  def initialize(elem)
    @text = elem.text
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
  f.puts("pub enum Resource {")
  protocols.each do |protocol|
    protocol.interfaces.each do |interface|
      f.puts("    #{camel_case(interface.name)}(super::#{interface.name}::#{camel_case(interface.name)}),")
    end
  end
  f.puts("}")
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
      if interface.enums
        f.puts("")
        f.print("pub mod enums {")
        interface.enums.each do |enum|
          f.puts("")
          f.puts("    pub enum #{camel_case(enum.name)} {")
          f.puts("    }")
        end
        f.puts("}")
      end
      if interface.events
        f.puts("")
        f.print("pub mod events {")
        interface.events.each do |event|
          f.puts("")
          f.puts("    pub struct #{camel_case(event.name)} {")
          f.puts("    }")
        end
        f.puts("}")
      end
      f.puts("")
      f.puts("pub struct #{camel_case(interface.name)} {")
      f.puts("}")
    end
  end
end
