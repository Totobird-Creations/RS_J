package net.totobirdcreations.robot;

import org.astonbitecode.j4rs.api.Instance;
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils;


public class Link
{

    static {
        String path  = Link.class.getProtectionDomain().getCodeSource().getLocation().getPath();
        path         = path.substring(0, path.lastIndexOf("/"));
        path         = path.replaceAll("%20"," ");
        path        += "/robot.dll";
        System.out.println(path);
        System.load(path);
    }


    private static native Instance rust_sum(Instance<Integer> a_instance, Instance<Integer> b_instance);


    public static Integer sum(Integer a, Integer b) {
        return Java2RustUtils.getObjectCasted(Link.rust_sum(
            Java2RustUtils.createInstance(a),
            Java2RustUtils.createInstance(b)
        ));
    }

}
