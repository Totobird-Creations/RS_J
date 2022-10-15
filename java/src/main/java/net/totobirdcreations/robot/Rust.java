package net.totobirdcreations.robot;

import org.astonbitecode.j4rs.api.Instance;
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils;


public class Rust
{
    static {
        String path  = Rust.class.getProtectionDomain().getCodeSource().getLocation().getPath();
        path         = path.substring(0, path.lastIndexOf("/"));
        path         = path.replaceAll("%20"," ");
        path        += "/robot.";
        String os    = System.getProperty("os.name").toLowerCase();
        if (os.contains("linux") || os.contains("unix")) {
            path += "so";
        } else if (os.contains("win")) {
            path += "dll";
        } else {
            throw new UnsatisfiedLinkError("Can not load dynamic library in unknown operating system `" + os + "`");
        }
        System.load(path);
    }


    public static native void hello();

    private static native Instance<Integer> rustSum(Instance<Integer> i1, Instance<Integer> i2);


    public static Integer sum(Integer a, Integer b) {
        return Java2RustUtils.getObjectCasted(Rust.rustSum(
            Java2RustUtils.createInstance(a),
            Java2RustUtils.createInstance(b)
        ));
    }

}
