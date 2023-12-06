package org.example;

import java.util.ArrayList;
import java.util.List;

public class Maps {

    private String Definition;
    private List<MapLine> lines;

    public Maps(String definition){
        this.Definition = definition;
        lines = new ArrayList<>();

    }

    public Maps(String definition, List<MapLine> lines){
        this.Definition = definition;
        this.lines = lines;
    }

    public void addMapLine(String line){

        String[] fields = line.split(" ");
        lines.add(new MapLine(fields[0],fields[1],fields[2]));

    }
    
    
    public long convertNumber(long number){
        for (MapLine ml  :lines) {

            if(number<= ml.getSourceRangeEnd() && number>= ml.getSourceRangeStart()){
                return number + ml.getDifference();
            }
            
        }
        return  number;
    }

}
