package net.totobirdcreations.robot.rust;

import org.astonbitecode.j4rs.api.Instance;
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils;


public class TestObject {

    public String content;


    public TestObject(String start) {
        this.content = start;
    }


    private static native Instance<TestObject> create(Instance<String> text);

    public static TestObject createFromRust(String next) {
        return Java2RustUtils.getObjectCasted(TestObject.create(
            Java2RustUtils.createInstance(next)
        ));
    }

}
