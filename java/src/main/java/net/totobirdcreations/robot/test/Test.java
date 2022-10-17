package net.totobirdcreations.robot.test;

import org.astonbitecode.j4rs.api.Instance;

import net.totobirdcreations.robot.RsHelper;


public class Test {
    
    public static native void hello();

    public static void world() {
        System.out.println("World!");
    }

    private static native Instance<Integer> add(Instance<Integer> a, Instance<Integer> b);
    public static Integer sum(Integer a, Integer b) {
        return RsHelper.rs2j(Test.add(
            RsHelper.j2rs(a),
            RsHelper.j2rs(b)
        ));
    }

}
