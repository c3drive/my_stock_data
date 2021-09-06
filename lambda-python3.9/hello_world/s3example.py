# apt install python-pip python3-pip
# pip3 install boto3
# python3 hello_world/s3example.py

import boto3
import tempfile

s3 = boto3.resource('s3')
s3_write_bucket = 'cf-templates-y9n0j1vbhsq-ap-northeast-1'

def lambda_handler(event, context):
    for rec in event['Records']:
        filename = rec['s3']['object']['key']
        s3_read_bucket = rec['s3']['bucket']['name']
        hello(s3_read_bucket, filename)
    return {
        "statusCode": 200,
    }


def hello(s3_read_bucket, filename):
    # ファイルの読み込み
    obj = s3.Object(s3_read_bucket, filename)
    response = obj.get()
    tmpdir = tempfile.TemporaryDirectory()
    fp = open(tmpdir.name + '/' + filename, 'wb')
    fp.write(response['Body'].read())
    fp.close;

    # ファイル名に.zipをつけてS3にアップロード
    zipname = tempfile.mkstemp()[1]
    obj = s3.Object(s3_write_bucket, filename + '.zip')
    response = obj.put(
        Body=open(tmpdir.name + '/' + filename, 'rb')
    )

    tmpdir.cleanup()
    return response


if __name__ == '__main__':
    print(hello("elasticbeanstalk-ap-northeast-1-428694349470", "port.png"))