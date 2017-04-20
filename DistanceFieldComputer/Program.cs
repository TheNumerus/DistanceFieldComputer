using System;
using System.Diagnostics;
using System.Drawing;
using System.Drawing.Imaging;
using System.IO;

namespace DistanceFieldComputer
{
    internal class Program
    {
        private static void Main(string[] args)
        {
            var g = new Generator();
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
            var SaveDirectory = Path.GetDirectoryName(args[0]);
            var savePath = SaveDirectory + "\\" + Path.GetFileNameWithoutExtension(args[0]) + "_output" + Path.GetExtension(args[0]);
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
                if (cki.Key == ConsoleKey.Escape) Environment.Exit(-1);
            } while (cki.Key != ConsoleKey.Enter);

            var sw = Stopwatch.StartNew();
            g.distanceField = new Bitmap(g.original.Width, g.original.Height,PixelFormat.Format24bppRgb);
            g.PrepareBitmaps();
            g.ComputePattern();
            Console.Write("\n");
            g.GetPoints();
            Console.Write("\n");
            g.GetDistances();
            Console.Write("\n");
            g.ComputeImage();
            Console.WriteLine("\n");
            g.distanceField.Save(savePath, g.original.RawFormat);

            Console.WriteLine("\nFinished in " + sw.ElapsedMilliseconds / 1000 + " seconds");
            Console.ReadLine();
        }
    }
}