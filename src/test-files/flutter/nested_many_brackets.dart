
import 'package:flutter/material.dart';

class MyApp extends StatelessWidget {

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
