using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;

namespace DistanceFieldComputer
{
    internal class Generator
    {
        public Bitmap distanceField = new Bitmap(1, 1);
        public float longest = float.MinValue;
        public Bitmap original = new Bitmap(1, 1);
        public List<Point> pattern = new List<Point>();
        public List<Point> points = new List<Point>();
        public float radius;

        public void ComputePattern()
        {
            for (var x = -(int) radius; x <= radius; x++)
            for (var y = -(int) radius; y <= radius; y++)
            {
                var point = new Point(x, y);
                point.ComputeDistanceToOrigin();
                if (point.distance <= radius)
                    pattern.Add(point);
                Console.Write("\r1/5 - Generating pattern {0}%, {1}/{2} finished               ", Math.Round(((x + radius) * (2 * radius + 1) + (y + radius)) / Math.Pow(2 * radius + 1, 2) * 100.0f), (x + radius) * (2 * radius + 1) + (y + radius), Math.Pow(2 * radius + 1, 2));
            }
            Console.Write("\n2/5 - Sorting pattern                            ");
            pattern = pattern.OrderBy(o => o.distance).ToList();
        }

        public void GetPoints()
        {
            for (var x = 0; x < original.Width; x++)
            for (var y = 0; y < original.Height; y++)
            {
                if (original.GetPixel(x, y).GetBrightness() > 0.5f)
                    points.Add(new Point(x, y));
                Console.Write("\r3/5 - Getting valid points {0}%, {1}/{2} finished               ", Math.Round((x * original.Height + y) / (float) (original.Height * original.Width) * 100.0f), x * original.Height + y, original.Height * original.Width);
            }
        }

        public void GetDistances()
        {
            foreach (var validPixel in points)
            {
                var distance = float.NaN;
                foreach (var point in pattern)
                    if (IsPixelBlack(validPixel.x + point.x, validPixel.y + point.y))
                    {
                        distance = point.distance;
                        if (point.distance > longest)
                            longest = point.distance;
                        break;
                    }
                Console.Write("\r4/5 - Getting distances {0}%, {1}/{2} finished               ", Math.Round(points.IndexOf(validPixel) / (float) points.Count * 100.0f), points.IndexOf(validPixel), points.Count);
                validPixel.distance = distance;
            }
        }

        public void ComputeImage()
        {
            foreach (var point in points)
            {
                if (float.IsNaN(point.distance))
                    point.distance = longest;
                var color = Math.Min((int) Math.Round(point.distance / longest * 255), 255);
                distanceField.SetPixel(point.x, point.y, Color.FromArgb(255, color, color, color));
                Console.Write("\r5/5 - Computing image {0}%, {1}/{2} finished               ", Math.Round((float) points.IndexOf(point) / points.Count * 100.0f), (float) points.IndexOf(point), points.Count);
            }
        }

        private bool IsPixelBlack(int x, int y)
        {
            if (x <= 0 || x > original.Width - 1 || y <= 0 || y > original.Height - 1)
                return false;
            return original.GetPixel(x, y).GetBrightness() < 0.5;
        }
    }
}