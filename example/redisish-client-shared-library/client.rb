require 'ffi'

module MyLib
  extend FFI::Library
  ffi_lib './libredisish_client.so'
  attach_function :send_message, [ :string ], :void
  attach_function :retrieve_message, [ ], :string
end

MyLib.send_message 'test'
puts MyLib.retrieve_message
