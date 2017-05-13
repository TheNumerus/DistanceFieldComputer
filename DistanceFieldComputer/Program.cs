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
            //create new generator for distance field
            var g = new Generator();

            //check if user importer bitmap
            if (args.Length == 0)
            {
                Console.WriteLine("You need to specify bitmap location. Please drag & drop image onto executable, or pass link as first argument.");
                Console.ReadKey();
                Environment.Exit(-1);
            }
            //check if valid input image
            try
            {
                g.inputImage = new Bitmap(args[0]);
            }
            catch (Exception e)
            {
                Console.WriteLine("You entered invalid file. Program will now exit.");
                Console.ReadKey();
                Environment.Exit(-1);
            }
            //generate output name and get location
            var SaveDirectory = Path.GetDirectoryName(args[0]);
            var savePath = SaveDirectory + "\\" + Path.GetFileNameWithoutExtension(args[0]) + "_output" + Path.GetExtension(args[0]);

            //show info to user
            Console.WriteLine("You opened " + args[0]);
            Console.WriteLine("Size of that texture is : " + g.inputImage.Width + " x " + g.inputImage.Height);

            //get radius
            Console.Write("Enter search radius: ");
            string radiusString;
            do
            {
                radiusString = Console.ReadLine();
            } while (!float.TryParse(radiusString, out g.radius));

            //get threshold
            Console.Write("Enter search threshold: ");
            string thresholdString;
            do
            {
                thresholdString = Console.ReadLine();
            } while (!int.TryParse(thresholdString, out g.threshold));

            Console.WriteLine("Your file will be saved as " + savePath);
            Console.WriteLine("Continue?");
            ConsoleKeyInfo cki;
            //get user confirmation to continue
            do
            {
                cki = Console.ReadKey();
                if (cki.Key == ConsoleKey.Escape) Environment.Exit(-1);
            } while (cki.Key != ConsoleKey.Enter);

            //start timer and create output bitmap
            var sw = Stopwatch.StartNew();
            g.outputImage = new Bitmap(g.inputImage.Width, g.inputImage.Height,PixelFormat.Format24bppRgb);

            //process functions
            g.PrepareBitmaps();
            g.ComputePattern();
            Console.Write("\n");
            g.GetPoints();
            Console.Write("\n");
            g.GetDistances();
            Console.Write("\n");
            g.ComputeImage();
            Console.WriteLine("\n");

            //save image
            g.outputImage.Save(savePath, g.inputImage.RawFormat);

            Console.WriteLine("\nFinished in " + sw.ElapsedMilliseconds / 1000 + " seconds");
            Console.ReadLine();
        }
    }
}