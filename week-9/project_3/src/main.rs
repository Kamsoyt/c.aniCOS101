use std::fs::write;

fn main() {
    let names = ["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona",
                 "Adewale Jimoh Akanbi","Osazuwa Faith Etiyeye"];
    let ministries = ["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let zones = ["South West","North East","South South","South West","South East"];

    for i in 0..5 {
        println!("{} | {} | {} | {}", i+1, names[i], ministries[i], zones[i]);
    }

    let mut out = String::from("S/N,Name,Ministry,Geopolitical Zone\n");
    for i in 0..5 {
        out.push_str(&format!("{},{},{},{}\n", i+1, names[i], ministries[i], zones[i]));
    }

    write("commissioners.csv", out).unwrap();
}

