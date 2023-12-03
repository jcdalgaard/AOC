using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AOC3
{
    internal class Filereader
    {
        public static List<string> ReadTextFile()
        {
            string path = @"C:\source\AOC\2023\day 3\AOC3\example.txt";
            List<string> lines = new List<string>();
            using (StreamReader sr = File.OpenText(path))
            {
                string s = "";
                while ((s = sr.ReadLine()) != null)
                {
                    lines.Add(s);   
                }
                return lines;
            }

        }
    }
}
