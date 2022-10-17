package net.totobirdcreations.robot.rust;

import org.astonbitecode.j4rs.api.Instance;
import org.astonbitecode.j4rs.api.dtos.InvocationArg;
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;


public class ConvHelper {
    
    public static <T> Instance<T> j2rs(T from) {
        return Java2RustUtils.createInstance(from);
    }

    @SuppressWarnings("unchecked")
    public static <T> T rs2j(Instance<T> from) {
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
