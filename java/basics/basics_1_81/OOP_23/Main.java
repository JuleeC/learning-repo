package java.basics.basics_1_81.OOP_23;

public class Main {
    public static void main(String[] args) {

        Car myCar1 = new Car();
        Car myCar2 = new Car();

        System.out.println(myCar1.make);
        System.out.println(myCar1.model);


        myCar1.drive();
        myCar1.brake();
    }
}
