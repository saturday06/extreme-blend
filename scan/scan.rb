#!/usr/bin/env ruby

require "rexml/document"

class Protocol
  attr_reader :name

  def initialize(elem)
    @name = elem.attributes["name"]
    protocol.select { |child| child.node_type == :element }.each do |child|
      case child.name
      when "copyright"
        raise "Oops!" if @copyright
        @copyright = Copyright.new(child)
      when "interface"
        @interfaces ||= []
        @interfaces << Interface.new(child)
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
  attr_reader :name

  def initialize(elem)
    @name = elem.attributes["name"].split('_').collect(&:capitalize).join
    elem.select { |elem| elem.node_type == :element }.each do |child|
    case elem.name
    when "description"
#      p elem
    when "request"
      emit_request(protocol_out, elem)
    end
  end  
  end
end

class Request
  def initialize(elem)
  end
end

class Enum
  def initialize(elem)
  end
end

class Event
  def initialize(elem)
  end
end

class Description
  def initialize(elem)
  end
end

protocol = Protocol.new(
  REXML::Document.new(File.read(path)).elements.find do |elem|
    elem.node_type == :element && elem.name == "protocol"
  end
)

def emit_request(protocol_out, request_elem)
  name = request_elem.attributes["name"]
  p name
  request_elem.select { |elem| elem.node_type == :element }.each do |elem|
    case elem.name
    when "description"
#      p elem
    when "arg"
      p elem.attributes["name"]
    end
  end  
end

def emit_copyright(protocol_out, elem)
end

def emit_interface(global_out, protocol_out, protocol_elem)
  name = protocol_elem.attributes["name"].split('_').collect(&:capitalize).join
  p name
  protocol_elem.select { |elem| elem.node_type == :element }.each do |elem|
    case elem.name
    when "description"
#      p elem
    when "request"
      emit_request(protocol_out, elem)
    end
  end  
end

global_out = StringIO.new

[
  "/usr/share/wayland/wayland.xml",
  "/usr/share/wayland-protocols/stable/xdg-shell/xdg-shell.xml",
].each do |path|
  protocol_out = StringIO.new
  
  protocol = REXML::Document.new(File.read(path)).elements.find do |elem|
    elem.node_type == :element && elem.name == "protocol"
  end
  protocol.select { |elem| elem.node_type == :element }.each do |elem|
    case elem.name
    when "copyright"
      emit_copyright(protocol_out, elem)
    when "interface"
      emit_interface(global_out, protocol_out, elem)
    end
  end

  p protocol_out
end
