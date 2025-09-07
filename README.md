content = """# Coordenadas

O **Coordenadas** é um aplicativo escrito em Rust com interface GTK (GTK3 e GTK4 experimental) que analisa um arquivo de texto em busca de coordenadas geográficas (UTM, graus decimais, GMS) e exporta os resultados para uma planilha CSV.

O objetivo é acelerar tarefas repetitivas do meu trabalho ao mesmo tempo que sirvo de exercício prático para aprender Rust + GTK.

---

## 🧩 Funcionalidades

- Leitura de arquivos de texto simples (logs, relatórios, arquivos brutos, etc.)
- Detecção e parsing de tripas de coordenadas em diferentes formatos:
  - UTM
  - Graus decimais
  - Graus, minutos e segundos (GMS)
- Apresentação dos resultados em uma interface gráfica
- Exportação dos resultados para arquivo .csv
- Compatível com Linux e Windows
- Ramificação experimental com suporte a GTK4

---

## 🚀 Começando

### Requisitos

- Rust (versão estável mais recente recomendada)
- Cargo
- GTK3 instalado no sistema (e GTK4 se quiser testar a versão experimental)

No Linux, por exemplo, você pode instalar via apt ou pacman:

```bash
# Debian/Ubuntu
sudo apt-get install libgtk-3-dev

# Arch Linux
sudo pacman -S gtk3
