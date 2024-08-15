from flask import Flask, jsonify

app = Flask(__name__)

@app.route('/api/<version>/', methods=['GET'])
def get_tv_info(version):
    # Static response mimicking a real Samsung TV API
    response = {
        "device": {
            "FrameTVSupport": "false",
            "GamePadSupport": "true",
            "ImeSyncedSupport": "true",
            "OS": "Tizen",
            "TokenAuthSupport": "true",
            "VoiceSupport": "false",
            "countryCode": "US",
            "description": "Samsung DTV RCR",
            "developerIP": "0.0.0.0",
            "developerMode": "0",
            "duid": "uuid:eba60459-5329-4df3-8ee6-a507d6fb21f7",
            "firmwareVersion": "Unknown",
            "id": "uuid:eba60459-5329-4df3-8ee6-a507d6fb21f7",
            "ip": "192.168.68.68",
            "model": "18_KANTSU_UHD_BASIC",
            "modelName": "UN50NU6900",
            "name": "[TV] Samsung 6 Series (50)",
            "networkType": "wireless",
            "resolution": "3840x2160",
            "smartHubAgreement": "true",
            "ssid": "40:ed:00:15:7c:f7",
            "type": "Samsung SmartTV",
            "udn": "uuid:eba60459-5329-4df3-8ee6-a507d6fb21f7",
            "wifiMac": "FC:03:9F:40:2E:1C"
        },
        "id": "uuid:eba60459-5329-4df3-8ee6-a507d6fb21f7",
        "isSupport": "{\"DMP_DRM_PLAYREADY\":\"false\",\"DMP_DRM_WIDEVINE\":\"false\",\"DMP_available\":\"true\",\"EDEN_available\":\"true\",\"FrameTVSupport\":\"false\",\"ImeSyncedSupport\":\"true\",\"TokenAuthSupport\":\"true\",\"remote_available\":\"true\",\"remote_fourDirections\":\"true\",\"remote_touchPad\":\"true\",\"remote_voiceControl\":\"false\"}",
        "name": "[TV] Samsung 6 Series (50)",
        "remote": "1.0",
        "type": "Samsung SmartTV",
        "uri": "http://192.168.68.68:8001/api/v2/",
        "version": "2.0.25"
    }

    return jsonify(response)

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8001)
