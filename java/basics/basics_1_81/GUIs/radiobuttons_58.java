package java.basics.basics_1_81.GUIs;
import java.awt.*;
import java.awt.event.*;
import javax.swing.*;
public class radiobuttons_58{

    public static void main(String[] args) {

        // JRadioButton = One or more buttons in a grouping in which only 1 may be selected per group

        new MyFrameeee();

    }
}
// ***********************************************************************


class MyFrameeee extends JFrame implements ActionListener {

    JRadioButton pizzaButton;
    JRadioButton hamburgerButton;
    JRadioButton hotdogButton;
    ImageIcon pizzaIcon;
    ImageIcon hamburgerIcon;
    ImageIcon hotdogIcon;

    MyFrameeee() {
        this.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        this.setLayout(new FlowLayout());

        //pizzaIcon = new ImageIcon("pizza.png");
        //hamburgerIcon = new ImageIcon("hamburger.png");
        //hotdogIcon = new ImageIcon("hotdog.png");

        pizzaButton = new JRadioButton("pizza");
        hamburgerButton = new JRadioButton("hamburger");
        hotdogButton = new JRadioButton("hotdog");

        ButtonGroup group = new ButtonGroup();
        group.add(pizzaButton);
        group.add(hamburgerButton);
        group.add(hotdogButton);

        pizzaButton.addActionListener(this);
        hamburgerButton.addActionListener(this);
        hotdogButton.addActionListener(this);

        //pizzaButton.setIcon(pizzaIcon);
        //hamburgerButton.setIcon(hamburgerIcon);
        //hotdogButton.setIcon(hotdogIcon);

        this.add(pizzaButton);
        this.add(hamburgerButton);
        this.add(hotdogButton);
        this.pack();
        this.setVisible(true);
    }

    @Override
    public void actionPerformed(ActionEvent e) {
        if (e.getSource() == pizzaButton) {
            System.out.println("You ordered pizza!");
            System.out.println(e.getSource());
        } else if (e.getSource() == hamburgerButton) {
            System.out.println("You ordered a hamburger!");
            System.out.println(e.getSource());
        } else if (e.getSource() == hotdogButton) {
            System.out.println("You ordered a hotdog!");
            System.out.println(e.getSource());
        }


    }
}
