package net.totobirdcreations.robot.rust;

import org.astonbitecode.j4rs.api.Instance;
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils;


public class TestObject {

    public String content;


    public TestObject(String start) {
        this.content = start;
    }


    private static native void concat(Instance<TestObject> self, Instance<String> next);
    public void concatFromRust(String next) {
        TestObject.concat(
            Java2RustUtils.createInstance(this),
            Java2RustUtils.createInstance(next)
        );
    }

}
