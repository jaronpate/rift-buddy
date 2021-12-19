import axios from 'axios'

export default function(method, endpoint, body) {
    return new Promise(resolve => {	
        var options = {
            method: method,
            url: `http://ddragon.leagueoflegends.com${endpoint}`,
            headers: {
                'Accept': 'application/json'
            },
            data: body
        };

        axios(options)
        .then(function (response) {
            resolve(response.data);
        })
        .catch(function (error){
            resolve(undefined, error)
        })
    });
}
