using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Drawing;
using System.Drawing.Imaging;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;

namespace DistanceFieldComputer
{
    internal class Generator
    {
#region variable definitions
        //input image variables
        public Bitmap inputImage = new Bitmap(1, 1);
        private BitmapData inputData;
        private IntPtr inputPointer;
        private int inputSize;
        private byte[] inputValues;

        //output image variables
        public Bitmap outputImage = new Bitmap(1, 1);
        private BitmapData outputData;
        private IntPtr outputPointer;
        private int outputSize;
        private byte[] outputValues;
        
        //working variables
        public List<Point> pattern = new List<Point>();
        public ConcurrentQueue<Bucket> buckets = new ConcurrentQueue<Bucket>();
        private ManualResetEvent doneEvent = new ManualResetEvent(false);
        public int progress = 0;

        //and input variables
        public float radius = 32;
        public int threshold = 127;
        
        //image info
        public int width;
        public int height;
        public PixelFormat pf;
        #endregion
#region main methods
        public void PrepareBitmaps()
        {

            Rectangle rect = new Rectangle(0, 0, inputImage.Width, inputImage.Height);

            //prepare input data
            inputData = inputImage.LockBits(rect, ImageLockMode.ReadOnly, inputImage.PixelFormat);
            inputPointer = inputData.Scan0;
            inputSize = Math.Abs(inputData.Stride) * inputImage.Height;
            inputValues = new byte[inputSize];
            Marshal.Copy(inputPointer, inputValues, 0, inputSize);

            //prepare output data
            outputData = outputImage.LockBits(rect, ImageLockMode.ReadWrite, PixelFormat.Format24bppRgb);
            outputPointer = outputData.Scan0;
            outputSize = Math.Abs(inputData.Stride) * inputImage.Height;
            outputValues = new byte[outputSize];
            Marshal.Copy(outputPointer, outputValues, 0, outputSize);

            //set image info and prepare multithreading
            width = inputImage.Width;
            height = inputImage.Height;
            pf = inputImage.PixelFormat;
            ThreadPool.SetMaxThreads(Environment.ProcessorCount, Environment.ProcessorCount);
        }

        public void ComputePattern()
        {
            int totalPoints = (int)Math.Pow(2 * radius + 1, 2);
            int pointsSoFar;
            //get list of points sorted by shortest distance to center, this way we can speed up the process of finding pixels
            for (var x = -(int) radius; x <= radius; x++)
            for (var y = -(int) radius; y <= radius; y++)
            {
                Point point = new Point(x, y);
                //precompute distance to speed up the process later
                point.ComputeDistanceToOrigin();
                if (point.distance <= radius)
                    pattern.Add(point);
                pointsSoFar = (int)((x + radius) * (2 * radius + 1) + (y + radius)) +1;
                Console.Write("\r1/5 - Generating pattern {0}%, {1}/{2} finished               ", Math.Round((float)pointsSoFar / totalPoints * 100.0f),pointsSoFar,totalPoints);
            }
            Console.Write("\n2/5 - Sorting pattern                            ");
            pattern = pattern.OrderBy(o => o.distance).ToList();
        }

        public void GetPoints()
        {
            int x = (int)Math.Ceiling(width / radius);
            int y = (int)Math.Ceiling(height / radius);

            //create indexed bucket array for quicker access, 0 for black, 1 for white, 2 for nearby
            byte[,] indices = new byte[x,y];
            for (var _x = 0; _x < x; _x++) for (var _y = 0; _y < y; _y++) indices[_x, _y] = 0;

            //create buckets
            GetPrimaryBuckets(x, y, indices);       
            
            //now add buckets whict don't contain white pixels, but are close to one
            GetNeighbourBuckets(x, y, indices);
        }

        private void GetPrimaryBuckets(int x, int y,byte[,] indices)
        {
            //prepare helper variables
            int xCenter;
            int yCenter;
            int halfRadius = (int)Math.Ceiling(radius / 2);

            for (var _x = 0; _x < x; _x++)
            for (var _y = 0; _y < y; _y++)
            {
                Bucket bucket = new Bucket(_x, _y);
                xCenter = (int)((_x * radius) + halfRadius);
                yCenter = (int)((_y * radius) + halfRadius);
                //detect white pixel in the bucket because of optimization
                foreach (Point p in pattern)
                {
                    //discard points outside of the bucket
                    if ((p.x + xCenter) > xCenter + halfRadius || (p.x + xCenter) < xCenter - halfRadius || (p.y + yCenter) > yCenter + halfRadius || (p.y + yCenter) < yCenter - halfRadius)
                        continue;
                    if (!IsPixelBlack(xCenter + p.x, yCenter + p.y))
                    {
                        bucket.Fill(width, height, (int)radius);
                        buckets.Enqueue(bucket);
                        indices[_x, _y] = 1;
                        break;
                    }
                }
            }
            Console.Write("3/5 - 50% {0} primary buckets found.", buckets.Count);
        }

        private void GetNeighbourBuckets(int x, int y, byte[,] indices)
        {
            for (var _x = 0; _x < x; _x++)
            for (var _y = 0; _y < y; _y++)
            {
                //don't check for already usable buckets
                if (indices[_x, _y] != 0)
                    continue;
                for (var xOffset = -1; xOffset <= 1; xOffset++)
                for (var yOffset = -1; yOffset <= 1; yOffset++)
                {
                    //check for invalid bucket indices
                    if (0 > (_x + xOffset) || (_x + xOffset) > x || 0 > (_y + yOffset) || (_y + yOffset) > y)
                        continue;

                    //check for self and newly added buckets
                    if ((xOffset == 0 && yOffset == 0) || indices[_x, _y] == 2)
                        continue;

                    if (indices[_x + xOffset, _y + yOffset] == 1)
                    {
                        Bucket bucket = new Bucket(_x, _y);
                        bucket.Fill(width, height, (int)radius);
                        buckets.Enqueue(bucket);
                        indices[_x, _y] = 2;
                        continue;
                    }
                }
            }
            Console.Write("\r3/5 - 100% Buckets done, {0} total.  ", buckets.Count);
        }

        public void GetDistances()
        {
            progress = 0;
            foreach(Bucket bucket in buckets)
            {
                ThreadPool.QueueUserWorkItem(GetDistanceBucket, bucket);
            }
            //wait for threads to finish
            doneEvent.WaitOne();
        }

        private void GetDistanceBucket(object bucket)
        {
            foreach (Point p in ((Bucket)bucket).points)
            {
                int x = p.x;
                int y = p.y;

                var distance = float.NaN;
                //check for distance for one pixel, using sorted list of points to speed up the process
                foreach (var point in pattern)
                {
                    //discard pixel from outside the image
                    if (IsPixelOutOfImage(x + point.x, y + point.y))
                        continue;
                    //check for distances
                    if (IsPixelBlack(x, y) && !IsPixelBlack(x + point.x, y + point.y))
                    {
                        distance = point.distance;
                        break;
                    }
                    if (!IsPixelBlack(x, y) && IsPixelBlack(x + point.x, y + point.y))
                    {
                        distance = point.distance;
                        break;
                    }
                }
                p.distance = distance;
            }
            //update progess
            progress++;
            Console.Write("\r4/5 - Getting distances {0}%, {1}/{2} finished               ", Math.Round((float)progress / buckets.Count * 100), progress, buckets.Count);
            if (progress == buckets.Count) doneEvent.Set();
        }
        
        //TODO fix brightness offset
        public void ComputeImage()
        {
            doneEvent = new ManualResetEvent(false);

            progress = 0;
            foreach (Bucket bucket in buckets)
            {
                ThreadPool.QueueUserWorkItem(ComputeImageBucket, bucket);
            }

            //wait for threads to finish
            doneEvent.WaitOne();

            //unlock image data for saving
            Marshal.Copy(inputValues, 0, inputPointer, inputSize);
            inputImage.UnlockBits(inputData);
            
            Marshal.Copy(outputValues, 0, outputPointer, inputSize);
            outputImage.UnlockBits(outputData);
        }

        private void ComputeImageBucket(object bucket)
        {
            byte color = 0;
            foreach (Point point in ((Bucket)bucket).points)
            {
                if (!IsPixelBlack(point.x, point.y))
                {
                    //set white pixels
                    if (float.IsNaN(point.distance))
                        point.distance = radius;
                    color = (byte)Math.Min(((int)Math.Round(point.distance / radius * 255f) / 2f) + 127f, 255);
                    SetPixel(point.x, point.y, outputValues, color);
                }
                else
                {
                    //set black pixels
                    if (float.IsNaN(point.distance))
                        point.distance = radius;
                    color = (byte)Math.Min((int)Math.Round((1f - point.distance / radius) * 255f) / 2f, 255);
                    SetPixel(point.x, point.y, outputValues, color);
                }
            }
            //update progess
            progress++;
            Console.Write("\r5/5 - Computing image {0}%, {1}/{2} finished               ", Math.Round((float)progress / buckets.Count * 100.0f), progress, buckets.Count);
            if (progress == buckets.Count) doneEvent.Set();
        }
#endregion
#region helpers
        private bool IsPixelBlack(int x, int y)
        {
            if (x < 0 || x > width - 1 || y < 0 || y > height - 1)
                return false;

            return GetPixel(x, y, inputValues) < threshold;
        }

        private bool IsPixelOutOfImage(int x, int y)
        {
            if (x < 0 || x >= width || y < 0 || y >= height)
                return true;

            return false;
        }

        private byte GetPixel(int x, int y, byte[] image)
        {
            var position = x * width + y;
            switch (pf)
            {
                case PixelFormat.Format24bppRgb:
                    position *= 3;
                    return image[position];
                case PixelFormat.Format32bppArgb:
                    position *= 4;
                    position += 1;
                    return image[position];
                default:
                    Console.WriteLine(pf);
                    return 127;
                    //throw new NotImplementedException();
            }
        }

        private void SetPixel(int x, int y, byte[] image, byte value)
        {
            var position = (x * width + y)*3;
            image[position] = value;
            image[position + 1] = value;
            image[position + 2] = value;
        }
#endregion
    }
}