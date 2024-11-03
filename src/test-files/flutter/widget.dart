import 'package:flutter/material.dart';

// Main Function
void main() {
  runApp(MyApp());
}

// Stateless Widget
class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Code Splitter Test',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: HomeScreen(),
    );
  }
}

// Another Stateless Widget
class HomeScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Home')),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Text('Welcome to the Code Splitter Test!'),
            MyCustomButton(),
            SizedBox(height: 20),
            CounterWidget(),
          ],
        ),
      ),
    );
  }
}

// Stateless Custom Button Widget
class MyCustomButton extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: () {
        showAlert(context, "Hello from MyCustomButton!");
      },
      child: Text('Press Me'),
    );
  }
}

// Stateful Counter Widget
class CounterWidget extends StatefulWidget {
  @override
  _CounterWidgetState createState() => _CounterWidgetState();
}

class _CounterWidgetState extends State<CounterWidget> {
  int _count = 0;

  void _incrementCounter() {
    setState(() {
      _count++;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: <Widget>[
        Text('Counter: $_count'),
        ElevatedButton(
          onPressed: _incrementCounter,
          child: Text('Increment Counter'),
        ),
      ],
    );
  }
}

// Helper Function to show Alert Dialog
void showAlert(BuildContext context, String message) {
  showDialog(
    context: context,
    builder: (BuildContext context) {
      return AlertDialog(
        title: Text('Alert'),
        content: Text(message),
        actions: <Widget>[
          TextButton(
            onPressed: () {
              Navigator.of(context).pop();
            },
            child: Text('OK'),
          ),
        ],
      );
    },
  );
}

// Another Helper Function to get formatted date string
String getFormattedDate() {
  return DateTime.now().toIso8601String();
}

// Stateless Date Display Widget
class DateDisplayWidget extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Text('Current Date: ${getFormattedDate()}');
  }
}
