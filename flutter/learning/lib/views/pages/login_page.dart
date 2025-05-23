import 'package:flutter/material.dart';
import 'package:learning/views/widget_tree.dart';
import 'package:lottie/lottie.dart';

class LoginPage extends StatefulWidget {
  const LoginPage({super.key, required this.title});

  final String title;

  @override
  State<LoginPage> createState() => _LoginPageState();
}

class _LoginPageState extends State<LoginPage> {
  TextEditingController controllerPw = TextEditingController(text: "456");
  TextEditingController controllerEmail = TextEditingController(text: "123");
  //example of using a controller to get the value of the text field
  String confirmedEmail = "123";
  String confirmedPw = "456";

  @override
  void dispose() {
    //when the widget is removed from the widget tree
    controllerPw.dispose();
    controllerEmail.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    double widthScreen = MediaQuery.of(context).size.width;
    
    return Scaffold(
      appBar: AppBar(),
      body: Center(
        child: SingleChildScrollView(
          child: Padding(
            padding: const EdgeInsets.all(20.0),
            child: LayoutBuilder(
              builder: (context, constraints) {
                return FractionallySizedBox(
                  widthFactor: widthScreen > 500 ? 0.5 : 1,
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Lottie.asset(
                        "assets/lotties/welcome.json",
                        width: 250,
                        height: 250,
                      ),
                      TextField(
                        controller: controllerEmail,
                        decoration: InputDecoration(
                          labelText: "Enter your email",
                          hintText: "Email",
                          border: OutlineInputBorder(
                            borderRadius: BorderRadius.circular(10.0),
                          ),
                        ),
                        onEditingComplete: () {
                          setState(() {
                            //
                          });
                        },
                      ),
                      SizedBox(height: 10.0),
                      TextField(
                        controller: controllerPw,
                        decoration: InputDecoration(
                          labelText: "Enter your Password",
                          hintText: "Password",
                          border: OutlineInputBorder(
                            borderRadius: BorderRadius.circular(10.0),
                          ),
                        ),
                        onEditingComplete: () {
                          setState(() {
                            //
                          });
                        },
                      ),
                      SizedBox(height: 20.0),
                      FilledButton(
                        onPressed: () {
                          onLoginPressed();
                        },
                        style: FilledButton.styleFrom(
                          minimumSize: Size(double.infinity, 40.0),
                        ),
                        child: Text(widget.title),
                      ),
                      SizedBox(height: 150.0),
                    ],
                  ),
                );
              },
            ),
          ),
        ),
      ),
    );
  }

  void onLoginPressed() {
    if (confirmedEmail == controllerEmail.text &&
        confirmedPw == controllerPw.text) {
      Navigator.pushAndRemoveUntil(
        context,
        MaterialPageRoute(
          builder: (context) {
            return WidgetTree();
          },
        ),
        (route) => false,
      );
    }
  }
}
