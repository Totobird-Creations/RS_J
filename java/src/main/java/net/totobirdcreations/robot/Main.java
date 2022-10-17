package net.totobirdcreations.robot;

import net.totobirdcreations.robot.test.Test;


public class Main {
    static {
        RsHelper.load();
    }

    public static void main(String[] args) {
        Test.hello();
        System.out.println(Test.sum(12, 5));
    }
}
