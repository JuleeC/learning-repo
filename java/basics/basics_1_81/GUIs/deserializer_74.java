package java.basics.basics_1_81.GUIs;


import java.io.*;

public class deserializer_74 {
    public static void main(String[] args) throws IOException, ClassNotFoundException {
        User user = null;
        FileInputStream filein = new FileInputStream("C://Users//jules//Documents//GitHub//Java_learn//java.basics.basics_1_81.acces_modifier_35.GUIs//UserInfo.ser");
        ObjectInputStream in = new ObjectInputStream(filein);
        user = (User) in.readObject();
        in.close();
        filein.close();

        System.out.println(user.name);
        System.out.println(user.password);
    }
}

class User_ implements Serializable {
    String name;
    String password;

    public void sayHello() {
        System.out.println("Hello");
    }

}
