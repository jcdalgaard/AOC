package org.example;

import java.util.List;

public class MapCreator {
    private int startRow;
    private List<String> lines;

    public MapCreator(List<String> lines){
        startRow = 2;
        this.lines = lines;
    }
    public Maps createMaps(){


        Maps m =  new Maps(lines.get(startRow));
        startRow++;

        while(lines.get(startRow)!= "" && lines.get(startRow).length() >0) {
            System.out.println(lines.get(startRow));
            m.addMapLine(lines.get(startRow));
            startRow++;
            if (startRow == lines.size()-1){
                break;
            }

        }
        startRow ++;
        return m;
    }

}
