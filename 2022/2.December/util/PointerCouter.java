package util;
import java.util.HashMap;
import javax.naming.ldap.HasControls;

public class PointerCouter {
    

    public static int converter(String something){
        HashMap<String, Integer> hm = new HashMap<>();
        
        hm.put("X", 1);
        hm.put("Y", 2);
        hm.put("Z", 3);
        hm.put("A", 1);
        hm.put("B", 2);
        hm.put("C", 3);


            return hm.get(something);


    }
    public static int getPoints(String x, String y)
    {

        int num1 =  converter(x);
        int num2 =  converter(y);

        if(num1 == num2){

            return num2+3;
        } else if(((num2- (num1))==1 || (num2 == 1 && num1 ==3 ) )){

            return num2+6;
        } else {
            return num2;
        }




    }


    
}
