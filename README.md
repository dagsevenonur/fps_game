# FPS Oyun Projesi

## Teknoloji Stack
- **Programlama Dili**: Rust
- **Game Engine Framework**: Bevy
- **Grafik API**: Wgpu
- **Fizik Motoru**: Rapier
- **ECS**: Legion
- **Network**: Tokio
- **UI**: Egui/iced

## Proje Yapısı
```
mygame/
├── src/
│   ├── core/           # Temel oyun mantığı
│   │   ├── game.rs     # Oyun döngüsü ve state yönetimi
│   │   ├── input.rs    # Input handling
│   │   └── config.rs   # Oyun konfigürasyonu
│   │
│   ├── entities/       # Oyun varlıkları
│   │   ├── player.rs   # Oyuncu sınıfı
│   │   ├── weapons/    # Silah sistemleri
│   │   └── world.rs    # Dünya nesneleri
│   │
│   ├── graphics/       # Grafik işlemleri
│   │   ├── renderer.rs # Ana render sistemi
│   │   ├── shaders/    # Shader dosyaları
│   │   └── camera.rs   # Kamera sistemi
│   │
│   ├── physics/        # Fizik motoru
│   │   ├── collision.rs # Çarpışma sistemi
│   │   └── movement.rs  # Hareket sistemi
│   │
│   ├── networking/     # Çevrimiçi özellikler
│   │   ├── client.rs   # İstemci kodu
│   │   ├── server.rs   # Sunucu kodu
│   │   └── protocol.rs # Network protokolü
│   │
│   ├── ui/            # Kullanıcı arayüzü
│   │   ├── hud.rs     # Heads-up display
│   │   └── menu.rs    # Menü sistemleri
│   │
│   └── utils/         # Yardımcı fonksiyonlar
│       ├── math.rs    # Matematik işlemleri
│       └── logger.rs  # Loglama sistemi
│
├── assets/            # Oyun assetleri
│   ├── models/       # 3D modeller
│   ├── textures/     # Dokular
│   ├── sounds/       # Ses dosyaları
│   └── shaders/      # Shader dosyaları
│
├── tests/            # Test dosyaları
├── Cargo.toml        # Bağımlılıklar
└── README.md         # Bu dosya

```

## Kod Yazım Prensipleri
1. **Class Bazlı Yapı**
   - Her bir bileşen için ayrı bir modül
   - Açık ve net sınıf hiyerarşisi
   - İyi tanımlanmış trait'ler ve interface'ler

2. **Kod Optimizasyonu**
   - DRY (Don't Repeat Yourself) prensibi
   - Gereksiz kod tekrarından kaçınma
   - Performans odaklı implementasyon

3. **Kod Okunabilirliği**
   - Açık ve anlaşılır değişken/fonksiyon isimlendirmesi
   - Düzenli kod dokümantasyonu
   - Modüler ve bakımı kolay yapı

## Önemli Bileşenler

### Core
- Oyun döngüsü yönetimi
- State management
- Event sistemi

### Entities
- Player karakteri ve özellikleri
- Silah sistemleri ve özellikleri
- NPC ve dünya objeleri

### Graphics
- Modern grafik pipeline
- Shader sistemi
- Kamera kontrolü

### Physics
- Çarpışma tespiti
- Hareket sistemleri
- Fizik simülasyonu

### Networking
- Client-Server mimarisi
- State senkronizasyonu
- Lag compensation

### UI
- In-game HUD
- Menü sistemleri
- Kullanıcı girdisi yönetimi 