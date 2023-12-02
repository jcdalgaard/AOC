import * as fs from 'fs';
interface Game {
    name: string
    id: number
    sets: cubeReveal[][]
    countBlue: number
    countRed: number
    countGreen:number
    
}
interface cubeReveal {
    color: string
    value: number
}
let sum =  0
let sum2 = 0
const calculateColorCounts = (game: Game): Game => {
    let maxRed = 0;
    let maxGreen = 0;
    let maxBlue = 0;
    game.sets.forEach(set => {
        set.forEach(cube => {
            switch (cube.color) {
                case 'red':
                    maxRed = cube.value > maxRed ? cube.value : maxRed;
                    break;
                case 'green':
                    maxGreen = cube.value > maxGreen ? cube.value : maxGreen;
                    break;
                case 'blue':
                    maxBlue = cube.value > maxBlue ? cube.value : maxBlue;
                    break;
            }
        });
    });
    game.countRed = maxRed;
    game.countGreen = maxGreen;
    game.countBlue = maxBlue;
    return game
}
let gameList: Game[] = []
fs.readFile('example.txt', 'utf8', (err, data) => {  data.split("\n").map(item => { gameList.push(calculateColorCounts({name: item.split(":")[0], id:parseInt(item.split(":")[0].split(" ")[1]), sets:item.split(":")[1].split(";").map(x => {return  x.split(",").map(y => {return {color: y.replace("\r","").trim().split(" ")[1],value: parseInt(y.replace("\r","").trim().split(" ")[0])}})}), countBlue:0, countRed:0, countGreen:0}))})
    gameList.forEach(val => {
        sum+= (val.countBlue>14 || val.countGreen>13 || val.countRed >12 ) ? 0: val.id
        sum2+= val.countBlue*val.countGreen*val.countRed
    })
    console.log(sum)
    console.log(sum2)
});


