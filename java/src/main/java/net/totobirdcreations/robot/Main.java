package net.totobirdcreations.robot;

import net.totobirdcreations.robot.rust.J2RS;
import net.totobirdcreations.robot.rust.TestObject;

public class Main 
{
    public static void main(String[] args) {
        J2RS.hello();
        System.out.println(J2RS.sumFromRust(12, 5));

        TestObject test = new TestObject("starttext ");
        test.concatFromRust("thiswasaddedbyrust");
    }
}
