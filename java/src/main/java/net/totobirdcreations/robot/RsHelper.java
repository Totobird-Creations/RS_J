package net.totobirdcreations.robot;

import org.astonbitecode.j4rs.api.Instance;
import org.astonbitecode.j4rs.api.dtos.InvocationArg;
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;


public enum RsHelper {;
    private static boolean LOADED = false;

    public static void load() {
        if (RsHelper.LOADED) {
            throw new LinkageError("Can not convert types unless `RsHelper.load()` has been called.");
        }
        String os=System.getProperty("os.name").toLowerCase();String path=RsHelper.class.getProtectionDomain().getCodeSource().getLocation().getPath();path=path.substring(0,path.lastIndexOf("/")).replaceAll("%20"," ")+"/"+
        "robot";
        if (os.contains("linux") || os.contains("unix") || os.contains("android")) {
            path += ".so";
        } else if (os.contains("win")) {
            path += ".dll";
        }
        else {throw new LinkageError("Can not load dynamic library in unknown operating system `" + os + "`");}
        System.load(path);
        RsHelper.LOADED = true;
    }


    public static <T> Instance<T> j2rs(T from) {
        if (! RsHelper.LOADED) {
            throw new LinkageError("Can not convert types unless `RsHelper.load()` has been called.");
        }
        return Java2RustUtils.createInstance(from);
    }

    @SuppressWarnings("unchecked")
    public static <T> T rs2j(Instance<T> from) {
        if (! RsHelper.LOADED) {
            throw new LinkageError("Can not convert types unless `RsHelper.load()` has been called.");
        }
        if (from instanceof InvocationArg) {
            InvocationArg from_ia = ((InvocationArg)from);
            if (from_ia.isSerialized()) {
                ObjectMapper mapper = new ObjectMapper();
                try {
                    return mapper.<T>readValue(from_ia.getJson(), (Class<T>)(Class.forName(from_ia.getClassName())));
                } catch (JsonProcessingException | ClassNotFoundException e) {
                    e.printStackTrace();
                    System.exit(1);
                }
            }
        }
        return Java2RustUtils.getObjectCasted(from);
    }

}
