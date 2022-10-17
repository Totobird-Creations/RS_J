package net.totobirdcreations.robot.rust;

import org.astonbitecode.j4rs.api.Instance;


public class TestObject {

    public String content;


    private static native Instance<TestObject> create(Instance<String> text);
    public static TestObject createFromRust(String text) {
        return ConvHelper.rs2j(TestObject.create(
            ConvHelper.j2rs(text)
        ));
    }

    private static native Instance<TestObject> concat(Instance<TestObject> self, Instance<String> text);
    public TestObject concatFromRust(String text) {
        return ConvHelper.rs2j(TestObject.concat(
            ConvHelper.j2rs(this),
            ConvHelper.j2rs(text)
        ));
    }

}
