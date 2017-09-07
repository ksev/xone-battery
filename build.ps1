$env:RUSTFLAGS = "-Ctarget-cpu=native"

&("C:\Program Files (x86)\Windows Kits\10\bin\x64\rc.exe") /r res/application.rc
cargo rustc --release -- -Clink-args="/SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup res\application.res"