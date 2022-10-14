package net.totobirdcreations.robot;

import java.io.File;

import org.astonbitecode.j4rs.api.Instance;
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils;


public class Link
{

    static {
        String path = "C:\\Users\\kjoly2025\\Code\\Java\\RustBridgeTest\\rust\\target\\debug\\robot.dll";
        System.out.println(path);
        System.load(path);
    }


    private static native Instance<Integer> rust_sum(Instance<Integer> a_instance, Instance<Integer> b_instance);


    public static Integer sum(Integer a, Integer b) {
        return Java2RustUtils.getObjectCasted(Link.rust_sum(
            Java2RustUtils.createInstance(a),
            Java2RustUtils.createInstance(b)
        ));
    }

}
