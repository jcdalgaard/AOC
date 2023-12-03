namespace AOC3
{
    public class Sol
    {
        private List<List<char>> chars;
        public List<Tuple<int, int>> nearNodes;
        public List<Tuple<int, List<Tuple<int, int>>>> numberLocations;
        public List<List<Tuple<int, int>>> tuplesGears;

        public Sol(List<string> list)
        {

            this.chars = new List<List<char>>();
            this.nearNodes = new List<Tuple<int, int>>();
            this.numberLocations = new List<Tuple<int, List<Tuple<int, int>>>>();
            this.tuplesGears = new List<List<Tuple<int, int>>>();

            SetUpChars(list);
            CheckNumberLocations();
            FindAllNumberPositions();
        }
        private void SetUpChars(List<string> list)
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
                nearNodes.Add(new Tuple<int, int>(i, j - 1));
            }
            if (char.IsDigit(chars[i][j]))
            {
                nearNodes.Add(new Tuple<int, int>(i, j));
            }
            if (char.IsDigit(chars[i][j + 1]))
            {
                nearNodes.Add(new Tuple<int, int>(i, j + 1));
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


        private List<Tuple<int, int>> CheckTrioGears(int i, int j, List<Tuple<int, int>> tupless)
        {
            if (char.IsDigit(chars[i][j - 1]))
            {
                tupless.Add(new Tuple<int, int>(i, j - 1));
            }
            if (char.IsDigit(chars[i][j]))
            {
                tupless.Add(new Tuple<int, int>(i, j));
            }
            if (char.IsDigit(chars[i][j + 1]))
            {
                tupless.Add(new Tuple<int, int>(i, j + 1));
            }
            return tupless;



        }

        public void FindGears()
        {
            for (int i = 0; i < chars.Count; i++)
            {
                for (int j = 0; j < chars[i].Count; j++)
                {
                    char c = chars[i][j];
                    if (c == '*')
                    {

                        List<Tuple<int, int>> tupless = new List<Tuple<int, int>>();
                        if (i > 0)
                        {
                            tupless = CheckTrioGears(i - 1, j, tupless);

                        }
                        tupless = CheckTrioGears(i, j, tupless);
                        if (i < chars.Count - 1)
                        {
                            tupless = CheckTrioGears(i + 1, j, tupless);
                        }

                        tuplesGears.Add(tupless);
                    }

                }
            }
        }

        public void CalcGearRatio()
        {
            var sum = 0;
            foreach (var item in tuplesGears)
            {
                int number1 = 0;
                int number2 = 0;
                foreach (var item1 in item)
                {

                    foreach (var numbers in numberLocations)
                    {
                        if (numbers.Item2.Contains(item1))
                        {

                            int tupleValue = numbers.Item1;


                            if (number1 != tupleValue && number1 != 0)
                            {
                                number2 = numbers.Item1;
                            }
                            else
                            {
                                number1 = numbers.Item1;
                            }
                        }
                    }
                }
                sum += number1 * number2;

            }
            Console.WriteLine(sum);

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
                    else if (numberBuilder != string.Empty)
                    {
                        numberBuilder = AddToTuple(tup, numberBuilder);
                    }

                }
                if (numberBuilder != string.Empty)
                {
                    numberBuilder = AddToTuple(tup, numberBuilder);
                }
            }


        }

        private string AddToTuple(List<Tuple<int, int>> tup, string numberBuilder)
        {
            List<Tuple<int, int>> tuc = new List<Tuple<int, int>>();

            tuc.AddRange(tup);
            Tuple<int, List<Tuple<int, int>>> t = new Tuple<int, List<Tuple<int, int>>>(int.Parse(numberBuilder), tuc);

            numberLocations.Add(t);

            tup.Clear();
            numberBuilder = string.Empty;
            return numberBuilder;
        }

        public void GetSum()
        {
            var sum = 0;
            foreach (var item in numberLocations)
            {
                foreach (var item1 in item.Item2)
                {
                    if (nearNodes.Contains(item1))
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
