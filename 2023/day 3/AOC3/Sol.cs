using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Security.Cryptography.X509Certificates;
using System.Text;
using System.Threading.Tasks;

namespace AOC3
{
    public class Sol
    {
        private List<List<char>> chars;
        List<string> list;
        public List<Tuple<int, int>> tuples;
        private List<int> numberToSum;
        public List<Tuple<int, List<Tuple<int, int>>>> tuplesToSum;

        public Sol(List<string> list)
        {
            this.list = list;
            this.chars = new List<List<char>>();
            this.tuples = new List<Tuple<int, int>>();
            this.numberToSum = new List<int>();
            this.tuplesToSum = new List<Tuple<int, List<Tuple<int, int>>>>();
            SetUpChars();
            Console.WriteLine(chars.ToString());
            CheckNumberLocations();
            Console.WriteLine("DONE...");
        }
        private void SetUpChars()
        {
            foreach (var item in list)
            {
                chars.Add(item.ToCharArray().ToList());

            }

        }
        private void CheckTrio(int i, int j)
        {
            if (char.IsDigit(chars[i][j - 1]))
            {
                tuples.Add(new Tuple<int, int>(i, j - 1));
            }
            if (char.IsDigit(chars[i][j]))
            {
                tuples.Add(new Tuple<int, int>(i, j));
            }
            if (char.IsDigit(chars[i][j + 1]))
            {
                tuples.Add(new Tuple<int, int>(i, j + 1));
            }

        }

        public void CheckNumberLocations()
        {
            for (int i = 0; i < chars.Count; i++)
            {
                for (int j = 0; j < chars[i].Count; j++)
                {
                    char c = chars[i][j];
                    if (!char.IsDigit(c) && c != '.')
                    {
                        if (i > 0)
                        {
                            CheckTrio(i - 1, j);

                        }
                        CheckTrio(i, j);
                        if (i < chars.Count - 1)
                        {
                            CheckTrio(i + 1, j);
                        }


                    }

                }
            }

        }
        public void FindNumbers()
        {
            if (tuples.Count == 0)
            {
                return;
            }

            for (int i = 0; i < chars.Count; i++)

            {
                bool addItem = false;
                string numberBuilder = "";
                for (int j = 0; j < chars[i].Count; j++)
                {
                    char c = chars[i][j];
                    if (char.IsDigit(c))
                    {
                        numberBuilder += c;

                    }
                    if (tuples.Contains(new Tuple<int, int>(i, j)))
                    {
                        addItem = true;
                    }

                }


            }

        }

        public void FindAllNumberPositions()
        {
            for (int i = 0; i < chars.Count; i++)
            {
                List<Tuple<int, int>> tup = new List<Tuple<int, int>>();
                string numberBuilder = string.Empty;
                for (int j = 0; j < chars[i].Count; j++)
                {
                    char c = (char)chars[i][j];
                    if (char.IsDigit(c))
                    {
                        numberBuilder += c;
                        tup.Add(new Tuple<int, int>(i, j));
                    }
                    else if(numberBuilder != string.Empty)
                    {
                        List<Tuple<int, int>> tuc = new List<Tuple<int, int>>();

                        tuc.AddRange(tup);
                        Tuple<int, List<Tuple<int, int>>> t = new Tuple<int, List<Tuple<int, int>>>(int.Parse(numberBuilder),tuc);
                        
                        tuplesToSum.Add(t);

                        tup.Clear();
                        numberBuilder = string.Empty;
                    }

                }
                if(numberBuilder != string.Empty) {
                    List<Tuple<int, int>> tuc = new List<Tuple<int, int>>();

                    tuc.AddRange(tup);
                    Tuple<int, List<Tuple<int, int>>> t = new Tuple<int, List<Tuple<int, int>>>(int.Parse(numberBuilder), tuc);

                    tuplesToSum.Add(t);

                    tup.Clear();
                    numberBuilder = string.Empty;
                }
            }


        }
        public void GetSum()
        {
            var sum = 0;
            foreach (var item in tuplesToSum)
            {
                foreach (var item1 in item.Item2)
                {
                    if (tuples.Contains(item1))
                    {
                        sum += item.Item1;
                        break;
                    }
                    
                }


            }
            Console.WriteLine(sum);
        }
    }
}
