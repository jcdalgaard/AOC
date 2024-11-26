package util; 
import java.io.File;  // Import the File class
import java.io.FileNotFoundException;  // Import this class to handle errors
import java.util.Scanner; // Import the Scanner class to read text files
import util.PointerCouter;
public class App{

    
public static String getLetter(String letter1,String letter2){

String[] letters =  new String[]{"X","Y","Z"};


  if (letter2.equals("Y")) {

    return letter1;

  } else if(letter2.equals("Z")){

    int number  = PointerCouter.converter(letter1);
    return letters[(number%3)];

  }
  else {
    int number  = PointerCouter.converter(letter1);
    return letters[((number+1)%3)];


  }



}


public static void main(String[] args) {
    
    try {
        int total =  0;
        File myObj = new File("2.December/t.txt");
        Scanner myReader = new Scanner(myObj);
        while (myReader.hasNextLine()) {
          String line  = myReader.nextLine();
          
//            total += PointerCouter.getPoints(String.valueOf(line.charAt(0)),String.valueOf(line.charAt(2)));
            total += PointerCouter.getPoints(String.valueOf(line.charAt(0)),App.getLetter(String.valueOf(line.charAt(0)), String.valueOf(line.charAt(2))) );




            System.out.println(line+" " +total);
          }
        myReader.close();
      } catch (FileNotFoundException e) {
        System.out.println("An error occurred.");
        e.printStackTrace();
      }

}




}