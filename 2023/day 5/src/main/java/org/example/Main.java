package org.example;

import java.util.List;

public class Main {
    public static void main(String[] args) {

        ReadFile rf = new ReadFile("src/main/resources/example.txt");

        List<String> lines =  rf.getLines();
        List<String> seeds = List.of(lines.get(0).replace("seeds: ","").split(" "));

        MapCreator mc = new MapCreator(lines);

        Maps seedsToSoil =  mc.createMaps();
        Maps soilToFertilizer =  mc.createMaps();
        Maps fertilizerToWater =  mc.createMaps();
        Maps waterToLight =  mc.createMaps();
        Maps lightToTemperature =  mc.createMaps();
        Maps temperatureToHumidity =  mc.createMaps();
        Maps humiditytoLocation =  mc.createMaps();

        long lowestNumber = -1;
        for (String seed: seeds) {

            long start = Long.parseLong(seed);
            start = seedsToSoil.convertNumber(start);
            start = soilToFertilizer.convertNumber(start);
            start = fertilizerToWater.convertNumber(start);
            start = waterToLight.convertNumber(start);
            start = lightToTemperature.convertNumber(start);
            start = temperatureToHumidity.convertNumber(start);
            start = humiditytoLocation.convertNumber(start);
            if (lowestNumber==-1){
                lowestNumber =start;

            }else {
                lowestNumber = Math.min(lowestNumber,start);
            }
        }

        System.out.println(lowestNumber);

        System.out.println("Hope....");
    }
}