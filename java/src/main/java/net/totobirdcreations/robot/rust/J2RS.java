package net.totobirdcreations.robot.rust;

import org.astonbitecode.j4rs.api.Instance;
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils;


public class J2RS
{
    static {
        String os = System.getProperty("os.name")
            .toLowerCase();
        String path = J2RS.class
            .getProtectionDomain().getCodeSource()
            .getLocation().getPath();
        path = path
            .substring(0, path.lastIndexOf("/"))
            .replaceAll("%20"," ")
            + "/robot.";
        if (os.contains("linux") || os.contains("unix") || os.contains("android")) {
            path += "so";
        } else if (os.contains("win")) {
            path += "dll";
        } else {
            throw new UnsatisfiedLinkError("Can not load dynamic library in unknown operating system `" + os + "`");
        }
        System.load(path);
    }


    public static native void hello();

    private static native Instance<Integer> sum(Instance<Integer> i1, Instance<Integer> i2);


    public static Integer sumFromRust(Integer a, Integer b) {
        return Java2RustUtils.getObjectCasted(J2RS.sum(
            Java2RustUtils.createInstance(a),
            Java2RustUtils.createInstance(b)
        ));
    }

}
