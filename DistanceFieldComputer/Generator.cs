using System;
using System.Collections.Generic;
using System.Drawing;
using System.Drawing.Imaging;
using System.Linq;
using System.Runtime.InteropServices;

namespace DistanceFieldComputer
{
    internal class Generator
    {
        private int bytes;
        public Bitmap distanceField = new Bitmap(1, 1);
        public float longest = float.MinValue;
        private BitmapData newData;
        private byte[] newValues;
        public Bitmap original = new Bitmap(1, 1);
        private BitmapData originalData;

        private byte[] origValues;
        public List<Point> pattern = new List<Point>();
        public List<Point> points = new List<Point>();
        private IntPtr ptr;
        private IntPtr ptrnew;
        public float radius;

        public void PrepareBitmaps()
        {
            var rect = new Rectangle(0, 0, original.Width, original.Height);

            originalData = original.LockBits(rect, ImageLockMode.ReadOnly, original.PixelFormat);
            ptr = originalData.Scan0;
            bytes = Math.Abs(originalData.Stride) * original.Height;
            origValues = new byte[bytes];
            Marshal.Copy(ptr, origValues, 0, bytes);

            newData = distanceField.LockBits(rect, ImageLockMode.ReadWrite, original.PixelFormat);
            ptrnew = newData.Scan0;
            newValues = new byte[bytes];
            Marshal.Copy(ptrnew, newValues, 0, bytes);
            /*for (int counter = 2; counter < origValues.Length; counter += 3)
                origValues[counter] = 255;*/
        }

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
                if (!IsPixelBlack(x, y))
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
                SetPixel(point.x, point.y, newValues, Color.FromArgb(255, color, color, color));
                Console.Write("\r5/5 - Computing image {0}%, {1}/{2} finished               ", Math.Round((float) points.IndexOf(point) / points.Count * 100.0f), (float) points.IndexOf(point), points.Count);
            }
            Marshal.Copy(origValues, 0, ptr, bytes);
            original.UnlockBits(originalData);

            Marshal.Copy(newValues, 0, ptrnew, bytes);
            distanceField.UnlockBits(newData);
        }

        private bool IsPixelBlack(int x, int y)
        {
            if (x <= 0 || x > original.Width - 1 || y <= 0 || y > original.Height - 1)
                return false;

            return GetPixel(x, y, origValues) < 127;
        }

        private byte GetPixel(int x, int y, byte[] image)
        {
            var position = x * original.Width + y;
            switch (original.PixelFormat)
            {
                case PixelFormat.Format24bppRgb:
                    position *= 3;
                    return image[position];
                case PixelFormat.Format32bppArgb:
                    position *= 4;
                    position += 1;
                    return image[position];
                default:
                    Console.WriteLine(original.PixelFormat);
                    throw new NotImplementedException();
            }
        }

        private void SetPixel(int x, int y, byte[] image, Color color)
        {
            var position = x * original.Width + y;
            switch (original.PixelFormat)
            {
                case PixelFormat.Format24bppRgb:
                    position *= 3;
                    image[position] = color.B;
                    image[position + 1] = color.G;
                    image[position + 2] = color.R;
                    break;

                case PixelFormat.Format32bppArgb:
                    position *= 4;
                    image[position] = 255;
                    image[position + 1] = color.R;
                    image[position + 2] = color.G;
                    image[position + 3] = color.B;
                    break;

                default:
                    Console.WriteLine(original.PixelFormat);
                    throw new NotImplementedException();
            }
        }
    }
}