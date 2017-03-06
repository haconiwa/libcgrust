file "c-examples/a.out" => (Dir.glob("include/*.h") + Dir.glob("c-examples/*.c")) do
  Dir.chdir "c-examples" do
    sh "gcc *.c -I../include -L../target/debug -lcgrust"
  end
end

task :c_demo => "c-examples/a.out" do
  puts "Running c demo..."
  ENV['LD_LIBRARY_PATH'] = "target/debug"
  sh "./c-examples/a.out"
end
