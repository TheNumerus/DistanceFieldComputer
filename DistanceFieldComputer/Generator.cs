using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Drawing;
using System.Drawing.Imaging;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;

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
        public List<Point> distances = new List<Point>();
        private IntPtr ptr;
        private IntPtr ptrnew;
        public float radius;

        public int width;
        public int height;
        public PixelFormat pf;

        public void PrepareBitmaps()
        {
            var rect = new Rectangle(0, 0, original.Width, original.Height);

            originalData = original.LockBits(rect, ImageLockMode.ReadOnly, original.PixelFormat);
            ptr = originalData.Scan0;
            bytes = Math.Abs(originalData.Stride) * original.Height;
            origValues = new byte[bytes];
            Marshal.Copy(ptr, origValues, 0, bytes);

            newData = distanceField.LockBits(rect, ImageLockMode.ReadWrite, PixelFormat.Format24bppRgb);
            ptrnew = newData.Scan0;
            newValues = new byte[bytes];
            Marshal.Copy(ptrnew, newValues, 0, bytes);
            width = original.Width;
            height = original.Height;
            pf = original.PixelFormat;
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
            List<Point> local1 = new List<Point>();
            List<Point> local2 = new List<Point>();
            List<Point> local3 = new List<Point>();
            List<Point> local4 = new List<Point>();
            Parallel.Invoke(
                () =>
                {
                    GetPartPoints(0, 0, width / 2, height / 2,out local1);
                    points.AddRange(local1);
                },
                () =>
                {
                    GetPartPoints(width / 2, 0, width / 2, height / 2, out local2);
                    points.AddRange(local2);
                },
                () =>
                {
                    GetPartPoints(0, height / 2, width / 2, height / 2, out local3);
                    points.AddRange(local3);
                },
                () =>
                {
                    GetPartPoints(width / 2, height / 2, width / 2, height / 2, out local4);
                    points.AddRange(local4);
                }
            );
        }

        private void GetPartPoints(int startX,int startY, int sizeX,int sizeY,out List<Point> local)
        {
            local = new List<Point>();
            for (var x = startX; x < startX+sizeX; x++)
            for (var y = startY; y < startY+sizeY; y++)
            {
                if (!IsPixelBlack(x, y))
                {
                    Console.Write("\r3/5 - Getting valid points {0}%, {1}/{2} finished               ", Math.Round((points.Count) / (float)(height * width) * 100.0f), points.Count, height * width);
                    local.Add(new Point(x, y));
                }
            }
        }

        public void GetDistances()
        {
            List<Point> local1 = new List<Point>();
            List<Point> local2 = new List<Point>();
            List<Point> local3 = new List<Point>();
            List<Point> local4 = new List<Point>();
            Parallel.Invoke(
                () =>
                {
                    GetPartDistances(1, out local1);
                    distances.AddRange(local1);
                },
                () =>
                {
                    GetPartDistances(2, out local2);
                    distances.AddRange(local2);
                },
                () =>
                {
                    GetPartDistances(3, out local3);
                    distances.AddRange(local3);
                },
                () =>
                {
                    GetPartDistances(4, out local4);
                    distances.AddRange(local4);
                }
            );
        }

        private void GetPartDistances(int quarter, out List<Point> local)
        {
            local = new List<Point>();
            for (int i = (quarter-1)*points.Count; i < (points.Count/4)*quarter; i++)
            {
                int x = points[i].x;
                int y = points[i].y;
                var distance = float.NaN;
                foreach (var point in pattern)
                    if (IsPixelBlack(x + point.x, y + points[i].y))
                    {
                        distance = point.distance;
                        if (point.distance > longest)
                            longest = point.distance;
                        break;
                    }
                Console.Write("\r4/5 - Getting distances {0}%, {1}/{2} finished               ", Math.Round(i / (float)points.Count * 100.0f), i, points.Count);
                local.Add(new Point(x,y,distance));
            }
        }

        public void ComputeImage()
        {
            foreach (var point in points)
            {
                if (float.IsNaN(point.distance))
                    point.distance = longest;
                var color = (byte)Math.Min((int) Math.Round(point.distance / longest * 255), 255);
                SetPixel(point.x, point.y, newValues,color);
                Console.Write("\r5/5 - Computing image {0}%, {1}/{2} finished               ", Math.Round((float) points.IndexOf(point) / points.Count * 100.0f), (float) points.IndexOf(point), points.Count);
            }
            Marshal.Copy(origValues, 0, ptr, bytes);
            original.UnlockBits(originalData);

            Marshal.Copy(newValues, 0, ptrnew, bytes);
            distanceField.UnlockBits(newData);
        }

        private bool IsPixelBlack(int x, int y)
        {
            if (x <= 0 || x > width - 1 || y <= 0 || y > height - 1)
                return false;

            return GetPixel(x, y, origValues) < 127;
        }

        private byte GetPixel(int x, int y, byte[] image)
        {
            var position = x * width + y;
            switch (pf)
            {
                case PixelFormat.Format24bppRgb:
                    position *= 3;
                    return image[position];
                    break;
                case PixelFormat.Format32bppArgb:
                    position *= 4;
                    position += 1;
                    return image[position];
                    break;
                default:
                    Console.WriteLine(pf);
                    return 127;
                    //throw new NotImplementedException();
            }
        }

        private void SetPixel(int x, int y, byte[] image, byte value)
        {
            var position = (x * original.Width + y)*3;
            image[position] = value;
            image[position + 1] = value;
            image[position + 2] = value;
        }
    }
}