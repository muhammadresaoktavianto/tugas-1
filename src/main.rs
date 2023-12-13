use std::collections::{HashMap, HashSet};
use std::io;
use std::io::Write; // Diperlukan untuk flush stdout

// Struktur data untuk merepresentasikan informasi wisata
struct Wisata {
    nama: String,
    keterangan: String,
    jam_operasional: (String, String), // Jam buka dan jam tutup
    hari_operasional: HashSet<String>,
    tarif_tiket: f64,
    lokasi: String,
}

// Fungsi utama program
fn main() {
    // Inisialisasi HashMap untuk menyimpan data wisata
    let mut data_wisata: HashMap<usize, Wisata> = HashMap::new();
    // Counter untuk menghasilkan ID unik untuk setiap wisata
    let mut counter = 1;

    // Loop utama program
    loop {
        // Menampilkan menu aplikasi
        println!("===== Aplikasi Pariwisata =====");
        println!("1. Tambah Data");
        println!("2. Lihat Data");
        println!("3. Edit Data");
        println!("4. Hapus Data");
        println!("5. Keluar");

        // Membaca input pengguna
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");

        // Memproses pilihan pengguna
        match input.trim().parse() {
            Ok(choice) => match choice {
                1 => tambah_data(&mut data_wisata, &mut counter),
                2 => lihat_data(&data_wisata),
                3 => edit_data(&mut data_wisata),
                4 => hapus_data(&mut data_wisata),
                5 => {
                    println!("Keluar dari aplikasi.");
                    break;
                }
                _ => println!("Pilihan tidak valid."),
            },
            Err(_) => println!("Masukkan angka yang valid."),
        }
    }
}

// Fungsi untuk menambahkan data wisata baru
fn tambah_data(data_wisata: &mut HashMap<usize, Wisata>, counter: &mut usize) {
    // Menampilkan judul dan mengambil input dari pengguna
    println!("===== Tambah Data =====");

    // Memasukkan nama tempat wisata
    let nama = input("Masukkan nama tempat wisata:");

    // Memasukkan keterangan tempat wisata
    let keterangan = input("Masukkan keterangan tempat wisata:");

    // Memasukkan jam operasional tempat wisata
    let jam_buka = input("Masukkan jam buka tempat wisata (format: HH:mm):");
    let jam_tutup = input("Masukkan jam tutup tempat wisata (format: HH:mm):");

    // Memasukkan hari operasional tempat wisata
    let hari_operasional = input("Masukkan hari operasional tempat wisata (pisahkan dengan tanda - , contoh: Senin-Selasa):")
        .split('-')
        .map(|s| s.trim().to_string())
        .collect();

    // Memasukkan tarif tiket tempat wisata
    let tarif_tiket = input_f64("Masukkan tarif tiket tempat wisata:");

    // Memasukkan lokasi tempat wisata
    let lokasi = input("Masukkan lokasi tempat wisata:");

    // Membuat instans Wisata baru dan menyimpannya di HashMap
    let wisata_baru = Wisata {
        nama,
        keterangan,
        jam_operasional: (jam_buka, jam_tutup),
        hari_operasional,
        tarif_tiket,
        lokasi,
    };

    data_wisata.insert(*counter, wisata_baru);
    *counter += 1;

    println!("Data berhasil ditambahkan.\n");
}

// Fungsi untuk mengedit data wisata
fn edit_data(data_wisata: &mut HashMap<usize, Wisata>) {
    // Menampilkan judul
    println!("===== Edit Data =====");
    // Memeriksa apakah HashMap kosong
    if data_wisata.is_empty() {
        println!("Tidak ada data untuk diedit.");
    } else {
        // Meminta pengguna memasukkan ID data yang ingin diedit
        println!("Masukkan ID data yang ingin diedit:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");

        // Memeriksa apakah ID dapat di-parse sebagai usize
        match input.trim().parse() {
            Ok(id) => {
                // Memeriksa apakah ID ada dalam HashMap
                if let Some(wisata) = data_wisata.get_mut(&id) {
                    // Meminta pengguna memasukkan data baru
                    println!("Masukkan nama tempat wisata baru:");
                    let mut nama = String::new();
                    io::stdin().read_line(&mut nama).expect("Gagal membaca input");

                    println!("Masukkan keterangan tempat wisata baru:");
                    let mut keterangan = String::new();
                    io::stdin().read_line(&mut keterangan).expect("Gagal membaca input");

                    println!("Masukkan lokasi tempat wisata baru:");
                    let mut lokasi = String::new();
                    io::stdin().read_line(&mut lokasi).expect("Gagal membaca input");

                    println!("Masukkan tarif tiket tempat wisata baru:");
                    let mut tarif_tiket = String::new();
                    io::stdin().read_line(&mut tarif_tiket).expect("Gagal membaca input");
                    let tarif_tiket: f64 = tarif_tiket.trim().parse().unwrap_or(0.0);

                    println!("Masukkan hari operasional tempat wisata baru (pisahkan dengan tanda - , contoh: Senin-Selasa):");
                    let mut hari_operasional = String::new();
                    io::stdin().read_line(&mut hari_operasional).expect("Gagal membaca input");
                    let hari_operasional: HashSet<String> =
                        hari_operasional.trim().split('-').map(|s| s.trim().to_string()).collect();

                    println!("Masukkan jam buka tempat wisata baru (format: HH:mm):");
                    let mut jam_buka = String::new();
                    io::stdin().read_line(&mut jam_buka).expect("Gagal membaca input");

                    println!("Masukkan jam tutup tempat wisata baru (format: HH:mm):");
                    let mut jam_tutup = String::new();
                    io::stdin().read_line(&mut jam_tutup).expect("Gagal membaca input");

                    // Mengupdate data wisata
                    wisata.nama = nama.trim().to_string();
                    wisata.keterangan = keterangan.trim().to_string();
                    wisata.lokasi = lokasi.trim().to_string();
                    wisata.tarif_tiket = tarif_tiket;
                    wisata.hari_operasional = hari_operasional;
                    wisata.jam_operasional = (jam_buka.trim().to_string(), jam_tutup.trim().to_string());

                    println!("Data berhasil diubah.\n");
                } else {
                    println!("ID tidak ditemukan.");
                }
            }
            Err(_) => println!("Masukkan angka yang valid."),
        }
    }
}

// Fungsi untuk menampilkan data wisata
fn lihat_data(data_wisata: &HashMap<usize, Wisata>) {
    // Menampilkan judul
    println!("===== Lihat Data =====");
    // Memeriksa apakah HashMap kosong
    if data_wisata.is_empty() {
        println!("Tidak ada data.");
    } else {
        // Menentukan lebar kolom "Lokasi" berdasarkan panjang string terpanjang
        let max_lokasi_length = data_wisata
            .values()
            .map(|wisata| wisata.lokasi.len())
            .max()
            .unwrap_or(10); // Set minimum lebar kolom "Lokasi" menjadi 10

        // Menampilkan header tabel
        println!(
            "{:<6} | {:<20} | {:<20} | {:<width$} | {:<15} | {:<20} | {:<20}",
            "ID",
            "Nama",
            "Keterangan",
            "Lokasi",
            "Tarif Tiket",
            "Hari Operasional",
            "Jam Operasional",
            width = max_lokasi_length
        );
        // Menampilkan garis pembatas tabel
        println!(
            "---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------"
        );
        // Menampilkan data wisata dalam format tabel
        for (id, wisata) in data_wisata {
            println!(
                "{:<6} | {:<20} | {:<20} | {:<width$} | {:<15} | {:<20} | {:<20}",
                id,
                wisata.nama,
                wisata.keterangan,
                wisata.lokasi,
                wisata.tarif_tiket,
                wisata.hari_operasional.iter().cloned().collect::<Vec<String>>().join("-"),
                format!("{} - {}", wisata.jam_operasional.0, wisata.jam_operasional.1),
                width = max_lokasi_length
            );
            // Menampilkan garis pembatas setiap baris tabel
            println!(
             "---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------"
            );
        }
    }
}

// Fungsi untuk menghapus data wisata
fn hapus_data(data_wisata: &mut HashMap<usize, Wisata>) {
    // Menampilkan judul
    println!("===== Hapus Data =====");
    // Memeriksa apakah HashMap kosong
    if data_wisata.is_empty() {
        println!("Tidak ada data untuk dihapus.");
    } else {
        // Meminta pengguna memasukkan ID data yang ingin dihapus
        println!("Masukkan ID data yang ingin dihapus:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");

        // Memeriksa apakah ID dapat di-parse sebagai usize
        match input.trim().parse() {
            Ok(id) => {
                // Memeriksa apakah ID ada dalam HashMap dan menghapusnya jika ada
                if let Some(_) = data_wisata.remove(&id) {
                    println!("Data berhasil dihapus.\n");
                } else {
                    println!("ID tidak ditemukan.");
                }
            }
            Err(_) => println!("Masukkan angka yang valid."),
        }
    }
}

// Fungsi utilitas untuk meminta input dari pengguna
fn input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Gagal membaca input");
    input.trim().to_string()
}

// Fungsi utilitas untuk meminta input f64 dari pengguna
fn input_f64(prompt: &str) -> f64 {
    loop {
        match input(prompt).trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("Masukkan angka yang valid."),
        }
    }
}
