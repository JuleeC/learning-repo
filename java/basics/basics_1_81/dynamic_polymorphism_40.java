package java.basics.basics_1_81;

import java.util.Scanner;
public class dynamic_polymorphism_40 {

    public static void main(String[] args) {

        //Dynamic Polymorphism
        //dynamic = after compilation (during runtime)

        Scanner scanner = new Scanner(System.in);
        Animal animal;

        System.out.println("What animal do you want?");
        System.out.print("(1=dog) or (2=cat): ");
        int choice = scanner.nextInt();

        if(choice==1) {
            animal = new Dog();
            animal.speak();
        }
        else if(choice==2) {
            animal = new Cat();
            animal.speak();
        }
        else {
            animal = new Animal();
            System.out.println("That choice was invalid");
            animal.speak();
        }
    }
}
//*************************************************
class Animal {

    public void speak() {
        System.out.println("animal goes *brrrr*");
    }
}
class Dog extends Animal{

    @Override
    public void speak() {
        System.out.println("dog goes *bark*");
    }
}
class Cat extends Animal{

    @Override
    public void speak() {
        System.out.println("cat goes *meow*");
    }
}