
package org.example;
import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

public class ReadFile {

    private List<String> lines;

    public ReadFile(String url){
        lines =  new ArrayList<>();
        try {
            BufferedReader reader = new BufferedReader(new FileReader(url));
            String line = reader.readLine();
            lines.add(line);
            while (line != null) {

                line = reader.readLine();

                lines.add(line);
            }
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public List<String> getLines(){
        return lines;
    }


}
