using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DistanceFieldComputer
{
    class Generator
    {
        public Bitmap original = new Bitmap(1,1);
        public Bitmap distanceField = new Bitmap(1, 1);
        public List<Point> points = new List<Point>();
        public List<Point> distances = new List<Point>();
        public List<Point> pattern = new List<Point>();
        public float radius;
        public float longest = float.MinValue;

        public void ComputeImage()
        {
            Console.Write("\n");
            foreach (Point distance in points)
            {
                if (float.IsNaN(distance.distance))
                {
                    distance.distance = longest;
                }
                int color = Math.Min((int)Math.Round((distance.distance / longest)*255), 255);
                distanceField.SetPixel(distance.x, distance.y, Color.FromArgb(255, color, color, color));
                Console.Write("\r1/1 - Computing image {0}%, finished               ", Math.Round((float)points.IndexOf(distance) / points.Count * 100.0f));
            }
        }

        public void GetDistances()
        {
            foreach (Point validPixel in points)
            {
                float distance=float.NaN;
                foreach (Point point in pattern)
                {
                    if (isPixelBlack(validPixel.x+point.x,validPixel.y+point.y))
                    {
                        distance = point.distance;
                        if (point.distance > longest)
                        {
                            longest = point.distance;
                        }
                        break;
                    }
                }
                Console.Write("\r1/1 - Getting distances {0}%, {1}/{2} finished               ", Math.Round((float)points.IndexOf(validPixel) / (float)points.Count * 100.0f),points.IndexOf(validPixel),points.Count);
                validPixel.distance = distance;
            }
        }

        public void GetPoints()
        {
            for (int x = 0; x < original.Width; x++)
            {
                for (int y = 0; y < original.Height; y++)
                {
                    if (original.GetPixel(x, y).GetBrightness() > 0.5f)
                    {
                        points.Add(new Point(x, y));
                    }
                    Console.Write("\r1/1 - Getting valid points {0}%, {1}/{2} finished               ", Math.Round((float)(x * original.Height + y) / (float)(original.Height * original.Width) * 100.0f),x*original.Height +y,original.Height*original.Width);
                }
            }
        }

        private static int Clamp(int value, int max, int min)
        {
            return Math.Max(min, Math.Min(value, max));
        }

        public void ComputePattern()
        {
            for (int x = -(int)radius; x <= radius; x++)
            {
                for (int y = -(int)radius; y <= radius; y++)
                {
                    Point point = new Point(x,y);
                    point.computeDistanceToOrigin();
                    if (point.distance<=radius)
                    {
                        pattern.Add(point);
                    }
                    Console.Write("\r1/2 - Generating {0}%, {1}/{2} finished               ", Math.Round((float)((x+radius) * (2*radius+1) + (y+radius)) / Math.Pow(2*radius+1,2) * 100.0f), ((x + radius) * (2 * radius + 1) + (y + radius)), Math.Pow(2 * radius + 1, 2));
                }
            }
            Console.Write("\n2/2 - Sorting                            ");
            pattern = pattern.OrderBy(o => o.distance).ToList();
        }

        private bool isPixelBlack(int x,int y)
        {
            if (x<=0 || x>original.Width-1 || y<=0 || y>original.Height-1)
            {
                return false;
            }
            if (original.GetPixel(x, y).GetBrightness() < 0.5)
                return true;
            return false;
        }
    }
}
