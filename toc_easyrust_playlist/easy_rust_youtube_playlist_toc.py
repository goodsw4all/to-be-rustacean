from googleapiclient.discovery import build
from datetime import timedelta
import re
import openpyxl

from openpyxl import Workbook
from openpyxl.cell import Cell
from openpyxl.styles import Font
import pandas as pd
import json

api_key = json.load(open(file='api_key'))['api_key']
youtube = build('youtube', 'v3', developerKey=api_key)

# Playlist Info
playlist_title = 'Easy Rust Korean / 한국어판'
playlist_description = ''
playlist_url = 'https://www.youtube.com/playlist?list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE'
playlist_duration = 110
playlist_video_count = 0
desc_done = False


def get_items_in_playlist(playlistId, df: pd.DataFrame = None):
    nextPageToken = None

    while True:
        playlist_req = youtube.playlistItems().list(
            part=['snippet', 'contentDetails'],
            playlistId=playlistId,
            maxResults=50,
            pageToken=nextPageToken
        )
        response = playlist_req.execute()
        global playlist_video_count
        playlist_video_count = response['pageInfo']['totalResults']

        for (idx, item) in enumerate(response['items']):
            title = item['snippet']['title']

            if "Private video" in title:
                print(item)
                continue

            index = title.find('!')
            title = title[index+1:].strip()
            index = title.find(':')
            title = title[0:4] + title[index+1:].strip()

            title = title.replace("EasyEasy Rust Korean", '').strip()

            try:
                int(title[0:3])
            except ValueError:
                continue

            global playlist_description
            global desc_done
            if idx == 0 and desc_done == False:
                print(item)
                playlist_description = item['snippet']['description']
                desc_done = True

            publishedAt = item['snippet']['publishedAt']
            # img_url = item['snippet']['thumbnails']['standard']['url']
            video_id = item['contentDetails']['videoId']
            video_url = f"{'https://www.youtube.com/watch?v='+video_id}"

            try:
                duration = get_video_duration(video_id)
            except:
                print(video_id)
                print(item)
            (hours, minutes, seconds, total_secs) = duration
            # playlist_duration += total_secs
            global playlist_duration
            playlist_duration += total_secs

            row = {
                'video_title': title,
                'video_url': video_url,
                'duration': f'{hours:02}:{minutes:02}:{seconds:02}',
                'published_at': publishedAt,
            }
            df = df.append(row, ignore_index=True)
            df.sort_values(by=['video_title'], inplace=True)
            # df = pd.concat([df, pd.DataFrame(row)])

        nextPageToken = response.get('nextPageToken')
        if not nextPageToken:
            break

    return df


def get_video_duration(video_id):
    hours_pattern = re.compile(r'(\d+)H')
    minutes_pattern = re.compile(r'(\d+)M')
    seconds_pattern = re.compile(r'(\d+)S')

    video_req = youtube.videos().list(
        part="contentDetails",
        id=video_id
    )
    vid_response = video_req.execute()
    duration = vid_response['items'][0]['contentDetails']['duration']
    hours = hours_pattern.search(duration)
    minutes = minutes_pattern.search(duration)
    seconds = seconds_pattern.search(duration)

    hours = int(hours.group(1)) if hours else 0
    minutes = int(minutes.group(1)) if minutes else 0
    seconds = int(seconds.group(1)) if seconds else 0

    total_secs = convert_hms_to_seconds(hours, minutes, seconds)

    return hours, minutes, seconds, int(total_secs)


def convert_hms_to_seconds(hours, minutes, seconds):
    total_secs = timedelta(
        hours=hours,
        minutes=minutes,
        seconds=seconds
    ).total_seconds()

    return total_secs


def convert_seconds_to_hms(total_seconds):
    return timedelta(seconds=total_seconds)


def set_cell_data(playlist_title, playlist_description, playlist_url, playlist_duration, playlist_df):
    print(playlist_description)
    writer = pd.ExcelWriter(
        'EasyRust_in_Korean.xlsx', engine='xlsxwriter')
    playlist_df.to_excel(writer, sheet_name="Easy Rust in Korean",
                         index=False, startrow=2, startcol=0)
    sheet = writer.sheets["Easy Rust in Korean"]

    pl_font_format = writer.book.add_format()

    wrap_format = writer.book.add_format({'text_wrap': True})
    wrap_format.set_align('center')
    wrap_format.set_align('vcenter')

    title_format = writer.book.add_format()

    cell_format = writer.book.add_format()
    cell_format.set_align('center')
    cell_format.set_align('vcenter')

    pl_cell_format = writer.book.add_format()
    pl_cell_format.set_align('vcenter')

    pl_cell_header_format = writer.book.add_format({'border': 1})
    pl_cell_header_format.set_bold()

    sheet.set_column('A:A', 60, title_format)
    sheet.set_column('B:B', 60, title_format)
    sheet.set_column('C:C', 25, cell_format)
    sheet.set_column('D:D', 70, wrap_format)

    sheet.write('A1', 'Title', pl_cell_header_format)
    sheet.write('A2', playlist_title, pl_cell_format)
    sheet.write('B1', 'Playlist Url', pl_cell_header_format)
    sheet.write_url('B2', playlist_url, pl_cell_format)
    sheet.write('C1', 'Stats', pl_cell_header_format)
    sheet.write(
        'C2',
        f"{convert_seconds_to_hms(playlist_duration)}\n{playlist_duration} secs\n\n{playlist_video_count} videos",
        wrap_format)
    sheet.write('D1', 'Description', pl_cell_header_format)
    sheet.write('D2', playlist_description)
    writer.save()


def set_font_all_cells():
    wb = openpyxl.load_workbook('EasyRust_in_Korean.xlsx')
    ws = wb["Easy Rust in Korean"]

    for row in ws.iter_cols(min_row=1, min_col=1, max_col=4):
        cell: Cell
        for cell in row:
            # print(cell.coordinate)
            cell.font = Font(size=15)
            # cell.alignment = Alignment(
            #     horizontal='center', vertical='center', wrap_text=True)

    wb.save(filename='EasyRust_in_Korean.xlsx')


cols = ['video_title', 'video_url', 'duration', 'published_at']
df = pd.DataFrame(columns=cols)

pl_id = "PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE"
playlist_df = get_items_in_playlist(pl_id, df)
set_cell_data(playlist_title, playlist_description,
              playlist_url, playlist_duration, playlist_df)
set_font_all_cells()
