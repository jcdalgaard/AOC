package org.example;

public class MapLine {
    private long destinationRangeStart;
    private long sourceRangeStart;
    private long rangeLentgh;

    private long difference;
    private long sourceRangeEnd;


    public MapLine(long destinationRangeStart, long sourceRangeStart, long rangeLentgh){
        this.destinationRangeStart = destinationRangeStart;
        this.sourceRangeStart = sourceRangeStart;
        this.rangeLentgh =  rangeLentgh;
        this.difference = this.destinationRangeStart-this.sourceRangeStart;
        this.sourceRangeEnd = this.sourceRangeStart+this.rangeLentgh-1;

    }
    public MapLine(String destinationRangeStart, String sourceRangeStart, String rangeLentgh){
        this.destinationRangeStart = Long.parseLong(destinationRangeStart);
        this.sourceRangeStart =Long.parseLong( sourceRangeStart);
        this.rangeLentgh = Long.parseLong( rangeLentgh);
        this.difference = this.destinationRangeStart-this.sourceRangeStart;
        this.sourceRangeEnd = this.sourceRangeStart+this.rangeLentgh-1;
    }

    public long getSourceRangeStart(){
        return this.sourceRangeStart;
    }
    public long getSourceRangeEnd(){
        return this.sourceRangeEnd;
    }
    public long getDifference(){
        return this.difference;
    }



}
