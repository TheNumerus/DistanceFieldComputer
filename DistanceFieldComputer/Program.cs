using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Drawing;
using System.Drawing.Drawing2D;
using System.IO;
using System.Linq;
using System.Security;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using System.Xml.Schema;

namespace DistanceFieldComputer
{
    class Program
    {
        static void Main(string[] args)
        {
            Generator g = new Generator();
            float radius;
            
            if (args.Length == 0)
            {
                Console.WriteLine("You need to specify bitmap location");
                Console.ReadKey();
                Environment.Exit(-1);
            }
            try
            {
                g.original = new Bitmap(args[0]);
            }
            catch (Exception e)
            {
                Console.WriteLine(e);
            }
            string SaveDirectory = Path.GetDirectoryName(args[0]);
            string savePath = SaveDirectory + "\\" + Path.GetFileNameWithoutExtension(args[0]) + "_output" + Path.GetExtension(args[0]);
            Console.WriteLine("You opened " + args[0]);
            Console.WriteLine("Size of that texture is : " + g.original.Width + " x " + g.original.Height);
            Console.Write("Enter search radius: ");
            string line;
            do
            {
                line = Console.ReadLine();
            } while (!float.TryParse(line, out radius));

            g.radius = radius;

            Console.WriteLine("Your file will be saved as " + savePath);
            Console.WriteLine("Continue?");
            ConsoleKeyInfo cki;
            do
            {
                cki = Console.ReadKey();
                if (cki.Key == ConsoleKey.Escape) Environment.Exit(-1); ;
            } while (cki.Key != ConsoleKey.Enter);

            Stopwatch sw = Stopwatch.StartNew();
            g.distanceField = new Bitmap(g.original.Width, g.original.Height);

            Console.WriteLine("\nComputing generator pattern");

            g.ComputePattern();

            Console.WriteLine("\nComputing generator pattern finished\n");
            Console.WriteLine("Getting all valid points");

            g.GetPoints();

            Console.WriteLine("\nGetting all valid points finished\n");
            Console.WriteLine("Computing distances");             

            g.GetDistances();

            Console.WriteLine("\nComputing distances finished\n");
            Console.WriteLine("Writing output file");

            g.ComputeImage();
            g.distanceField.Save(savePath, g.original.RawFormat);

            Console.WriteLine("\nFinished in " + sw.ElapsedMilliseconds / 1000 + " seconds");
            Console.ReadLine();
        }
        
    }
    public struct Distance
    {
        public int x;
        public int y;
        public double distance;
        public Distance(int x, int y, float distance)
        {
            this.x = x;
            this.y = y;
            this.distance = distance;
        }
    }
}
