namespace AOC3
{
    internal class Program
    {
        static void Main(string[] args)
        {
            Sol s = new Sol(Filereader.ReadTextFile());
            s.GetSum();

            s.FindGears();
            s.CalcGearRatio();

        }

    }
}