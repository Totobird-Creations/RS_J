@PushD ".\rust"
cargo build
@PopD
copy rust\target\debug\robot.dll java\resources\robot.dll
@PushD ".\java"
mvn clean package
@PopD